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
  // Persistent /home. Under QEMU there's a second virtio disk (/dev/vdb) holding
  // a data image that survives root-image rebuilds — mount it over /home so
  // files persist across `make run`. On the real Pi there's no second disk, so
  // this mount fails harmlessly (ENOENT) and /home stays on the SD root, which
  // persists across reboots on its own. Return value is intentionally ignored.
  mount("/dev/vdb", "/home", "ext4", 0, NULL);
  chdir("/home");

  // Environment for the whole user-session subtree. Set once on cinit;
  // every fork+exec child inherits via environ. Grouped by purpose:
  //
  //   XDG_*       — runtime dir + tells GTK we're in a graphical session
  //   GDK_BACKEND — force GTK to use Wayland (default would try X first)
  //   QT_QPA_*    — force Qt to Wayland; otherwise Qt asks XCB for X11
  //   SDL_*       — SDL2 apps (raylib's audio uses SDL on some builds)
  //   LIBGL_*     — Mesa: software-render first. Under QEMU `-M virt` there's
  //                 no host-GPU passthrough; on the real Pi 4 the VideoCore VI
  //                 (v3d/vc4) exists but we render via llvmpipe for bring-up.
  //                 zonule's GlesRenderer renders through Mesa/EGL, so these
  //                 LIBGL_/GALLIUM_ vars steer it onto llvmpipe.
  //                 >>> To enable the real Pi GPU later, drop LIBGL_ALWAYS_
  //                 SOFTWARE / GALLIUM_DRIVER here. <<<
  //   WLR_*       — LEGACY / no-ops now. The old compositor was wlroots-based;
  //                 zonule is Smithay and ignores WLR_*. Harmless, kept only as
  //                 a reminder; delete whenever. Software rendering is driven by
  //                 the LIBGL_/GALLIUM_ vars above, not these.
  //   LIBSEAT_BACKEND=builtin — run libseat's seat management IN-PROCESS (as
  //                 root) instead of via a seatd daemon. With SEATD_VTBOUND=0
  //                 (below) this is a single-seat, no-VT setup: zonule takes DRM
  //                 master directly. No seatd, no seatd-launch, no seatd.sock.
  //                 (The seatd *daemon* has no knob to become non-VT-bound in
  //                 this version, which is why the daemon path couldn't get
  //                 master on the serial console; the builtin backend does.)
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
  // The kernel has VT support (CONFIG_VT=y + fbcon), so libseat's builtin
  // backend would try to bind to the active VT to arbitrate DRM master. But
  // under QEMU `-M virt` we live on the serial console (ttyAMA0), never a
  // graphical VT, so that VT would never activate and every modeset would fail
  // with EPERM ("Unable to become drm master"). SEATD_VTBOUND=0 tells the
  // builtin backend to skip VT binding: as the sole root session, zonule takes
  // DRM master directly and fbcon (the boot-logo framebuffer console) hands the
  // display over. Correct for a single-seat appliance with no VT switching.
  setenv("LIBSEAT_BACKEND", "builtin", 1);
  setenv("SEATD_VTBOUND", "0", 1);
  // WLR_LIBINPUT_NO_DEVICES intentionally dropped: with virtio-gpu-pci and
  // the virtio kernel input drivers we now have keyboards/mice in /dev/input
  // that libinput should pick up.
  setenv("PATH", "/bin:/usr/bin:/usr/sbin", 1);
  setenv("HOME", "/root", 1);
  setenv("LANG", "en_US.UTF-8", 1);
  setenv("SHELL", "/bin/cjsh", 1);

  // Start the udev daemon. It's long-running, so don't wait on it here.
  if (fork() == 0) {
    char *argv[] = {"/lib/systemd/systemd-udevd", NULL};
    execvp(argv[0], argv);
    perror("udevd failed to start");
    exit(1);
  }

  // Coldplug: replay "add" uevents for devices that already exist, then block
  // until udev has drained its queue. `udevadm settle` replaces the old fixed
  // sleep(1) pair — it returns the instant processing finishes instead of
  // always burning a wall-clock second per step.
  {
    pid_t p;
    if ((p = fork()) == 0) {
      char *argv[] = {"/usr/bin/udevadm", "trigger", "-c", "add", NULL};
      execvp(argv[0], argv);
      perror("udevadm trigger failed");
      _exit(1);
    }
    waitpid(p, NULL, 0);
    if ((p = fork()) == 0) {
      char *argv[] = {"/usr/bin/udevadm", "settle", NULL};
      execvp(argv[0], argv);
      perror("udevadm settle failed");
      _exit(1);
    }
    waitpid(p, NULL, 0);
  }

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

  // Launch the compositor. zonule's -s flag runs a startup client once its
  // wayland socket is up: squishy (our Wayland-native terminal emulator), which
  // in turn execs cjsh — see $SHELL set above, which squishy reads. No
  // seatd-launch: zonule uses libseat's builtin backend directly (see env above).
  spawn((char *[]){"/bin/display", "-s", "/bin/squishy", NULL});

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
  // Respawn the compositor + squishy (our terminal emulator). squishy execs
  // $SHELL (=/bin/cjsh) inside the pty it opens. zonule uses libseat's builtin
  // backend (no seatd daemon), so there's no socket to clean up here.
  spawn((char *[]){"/bin/display", "-s", "/bin/squishy", NULL});
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
