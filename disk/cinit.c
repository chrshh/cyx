#include <stdlib.h>
#include <sys/ioctl.h>
#include <sys/types.h>
#include <sys/wait.h>
#include <sys/mount.h>
#include <sys/reboot.h>
#include <sys/stat.h>

#include <signal.h>
#include <stdio.h>
#include <unistd.h>
#include <fcntl.h>

#define LEN(x) (sizeof(x) / sizeof *(x))

static void sigpoweroff(void);
static void sigreap(void);
static void sigreboot(void);
static void spawn(char *const[]);

#ifndef RB_POWER_OFF
#define RB_POWER_OFF 0x4321fedc
#endif

static struct {
  int sig;
  void (*handler)(void);
} sigmap[] = {
    {SIGUSR1, sigpoweroff},
    {SIGCHLD, sigreap},
    {SIGINT, sigreboot},
};

static sigset_t set;

int main(void) {
  int sig;
  size_t i;

  if (getpid() != 1) return 1;
  mkdir("/home", 0755);
  // Filesystem type strings are kernel-internal names: "proc", "sysfs",
  // "devtmpfs". Wrong type → mount() returns -1 with ENODEV, silently if you
  // ignore the return value. devtmpfs is also auto-mounted by the kernel
  // when CONFIG_DEVTMPFS_MOUNT=y, so this call is a redundant safety net.
  mount("proc", "/proc", "proc", 0, NULL);
  mount("sysfs", "/sys", "sysfs", 0, NULL);

  mount("devtmpfs", "/dev", "devtmpfs", 0, NULL);

  mkdir("/dev/pts", 0755);
  mount("devpts", "/dev/pts", "devpts", 0, NULL);

  mkdir("/dev/shm", 0777);
  mount("tmpfs", "/dev/shm", "tmpfs", 0, NULL);

  mount("tmpfs", "/tmp", "tmpfs", 0, NULL);
  // /run holds seatd's socket and the per-user runtime dir for Wayland.
  // /run/user/0 is XDG_RUNTIME_DIR for root — required to be 0700 owned by
  // the user, mode is enforced by libwayland clients.
  mount("tmpfs", "/run", "tmpfs", 0, NULL);
  mkdir("/run/user", 0755);
  mkdir("/run/user/0", 0700);
  mount("/dev/vdb", "/home", "ext4", 0, NULL);
  chdir("/home");

  // Environment for the whole user-session subtree. Set once on cinit;
  // every fork+exec child inherits via environ. Grouped by purpose:
  //
  //   XDG_*       — runtime dir + tells GTK we're in a graphical session
  //   GDK_BACKEND — force GTK to use Wayland (default would try X first)
  //   QT_QPA_*    — force Qt to Wayland; otherwise Qt asks XCB for X11
  //   SDL_*       — SDL2 apps (raylib's audio uses SDL on some builds)
  //   LIBGL_*     — Mesa: no virgl/host-GPU passthrough on virtio-gpu-pci,
  //                 so force the llvmpipe software rasterizer
  //   WLR_*       — wlroots quirks left over from simpledrm experiments;
  //                 with virtio-gpu-pci we *could* drop these, but they're
  //                 harmless and keep the compositor predictable
  //   LIBSEAT_BACKEND is intentionally unset: seatd-launch sets SEATD_SOCK
  //                 and libseat auto-selects the seatd backend.
  setenv("XDG_RUNTIME_DIR", "/run/user/0", 1);
  setenv("XDG_SESSION_TYPE", "wayland", 1);
  setenv("GDK_BACKEND", "wayland", 1);
  setenv("QT_QPA_PLATFORM", "wayland", 1);
  setenv("SDL_VIDEODRIVER", "wayland", 1);
  setenv("CLUTTER_BACKEND", "wayland", 1);
  setenv("MOZ_ENABLE_WAYLAND", "1", 1);
  setenv("LIBGL_ALWAYS_SOFTWARE", "1", 1);
  setenv("GALLIUM_DRIVER", "llvmpipe", 1);
  setenv("WLR_RENDERER", "pixman", 1);
  setenv("WLR_NO_HARDWARE_CURSORS", "1", 1);
  // WLR_LIBINPUT_NO_DEVICES intentionally dropped: with virtio-gpu-pci and
  // the virtio kernel input drivers we now have keyboards/mice in /dev/input
  // that libinput should pick up.
  setenv("PATH", "/bin:/usr/bin:/usr/sbin", 1);
  setenv("HOME", "/root", 1);
  setenv("LANG", "en_US.UTF-8", 1);
  setenv("SHELL", "/bin/cjsh", 1);

  if (fork() == 0) {
    char *argv[] = {"/lib/systemd/systemd-udevd", NULL};
    execvp(argv[0], argv);
    perror("udevd failed to start");
    exit(1);
  }
  sleep(1);

  if (fork() == 0) {
    char *argv[] = {"/usr/bin/udevadm", "trigger", "-c", "add", NULL};
    execvp(argv[0], argv);
    perror("udevadm trigger failed");
    exit(1);
  }
  sleep(1);

  /**
   * @brief Opens the console and allocates file descriptors
   */
  int fd = open("/dev/console", O_RDWR);
  if (fd == -1) {
    perror("/dev/console failed to open");
    exit(EXIT_FAILURE);
  }

  dup2(fd, STDIN_FILENO);
  dup2(fd, STDOUT_FILENO);
  dup2(fd, STDERR_FILENO);
  close(fd);

  sigfillset(&set);
  sigprocmask(SIG_BLOCK, &set, NULL);

  // tinywl's -s flag runs a startup command via /bin/sh -c. We launch
  // squishy (our own Wayland-native terminal emulator), which in turn
  // execs cjsh — see $SHELL set above, which squishy reads.
  spawn((char *[]){"/usr/bin/seatd-launch", "--", "/bin/display",
                   "-s", "/bin/squishy", NULL});

  while (1) {
    sigwait(&set, &sig);
    for (i = 0; i < LEN(sigmap); i++) {
      if (sigmap[i].sig == sig) {
        sigmap[i].handler();
        break;
      }
    }
  }
  return 0;
}

static void sigpoweroff(void) {
  sync();
  reboot(RB_POWER_OFF);
}

/**
 * @note Once the shell is not the only child process,
 * I need to store cjsh's pid and only respawn when that pid is returned
 */
static void sigreap(void) {
  while (waitpid(-1, NULL, WNOHANG) > 0);
  // Stale socket left by a seatd that died without cleanup. Safe to unlink:
  // /run is tmpfs and the only writer is seatd-launch, which always recreates.
  unlink("/run/seatd.sock");
  // Respawn the compositor + squishy (our terminal emulator). squishy
  // execs $SHELL (=/bin/cjsh) inside the pty it opens.
  spawn((char *[]){"/usr/bin/seatd-launch", "--", "/bin/display",
                   "-s", "/bin/squishy", NULL});
}

static void sigreboot(void) {
  sync();
  reboot(RB_AUTOBOOT);
}

static void spawn(char *const argv[]) {
  switch (fork()) {
  case 0:
    sigprocmask(SIG_UNBLOCK, &set, NULL);
    setsid();
    // Manually assign a controlling terminal to the process spawned
    // ioctl(STDIN_FILENO, TIOCSCTTY, 0);
    execvp(argv[0], argv);
    perror("execvp");
    _exit(1);
  case -1:
    perror("fork");
  }
}
