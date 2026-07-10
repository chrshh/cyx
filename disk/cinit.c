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
#include <string.h>
#include <time.h>

#define LEN(x) (sizeof(x) / sizeof *(x))

static void sigpoweroff(void);
static void sigreap(void);
static void sigreboot(void);
static void spawn(char *const[]);
static void spawn_display(void);
static void apply_render_env(void);
static int cmdline_has(const char *token);

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

// Renderer selection. Default (software_render = 0): leave LIBGL_ALWAYS_SOFTWARE
// / GALLIUM_DRIVER unset so Mesa auto-picks a hardware driver — virgl under
// QEMU's virtio-gpu, v3d on a real Pi 4. If the compositor's first launch dies
// within a few seconds (the signature of a failed GL/EGL init), sigreap flips
// this to 1 and retries with llvmpipe software rendering. `cjyx.softrender` on
// the kernel cmdline sets it to 1 up front, skipping the hardware attempt.
static int software_render = 0;
static time_t display_started;

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
  //   LIBGL_/GALLIUM_ — renderer selection is DYNAMIC and is NOT set here.
  //                 zonule's GlesRenderer renders through Mesa/EGL; by default we
  //                 leave these unset so Mesa auto-picks a hardware driver (virgl
  //                 on QEMU's virtio-gpu, v3d on a real Pi 4). If that first
  //                 launch fails fast we fall back to llvmpipe — see
  //                 software_render / apply_render_env / sigreap. Force software
  //                 from boot with `cjyx.softrender` on the kernel cmdline
  //                 (`make run-g SOFTRENDER=1`).
  //   WLR_*       — LEGACY / no-ops now. The old compositor was wlroots-based;
  //                 zonule is Smithay and ignores WLR_*. Harmless, kept only as
  //                 a reminder; delete whenever.
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
  // LIBGL_ALWAYS_SOFTWARE / GALLIUM_DRIVER are applied later by apply_render_env
  // (hardware-first, software fallback), not pinned here.
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
  //
  // Renderer: try hardware (virgl/v3d) first; `cjyx.softrender` on the kernel
  // cmdline forces llvmpipe from the start, skipping the hardware attempt.
  if (cmdline_has("cjyx.softrender")) software_render = 1;
  spawn_display();

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
  // Renderer fallback: if we were attempting hardware rendering and the
  // compositor died within a few seconds of launch, that's almost certainly a
  // failed GL/EGL init (virgl/v3d unavailable), not a normal exit — switch to
  // llvmpipe software rendering for the retry. A compositor that ran a while and
  // then exited keeps the current renderer (this is just an ordinary respawn).
  // Our only direct child in the signal loop is /bin/display (squishy + cjsh are
  // its descendants), so a SIGCHLD here reliably means the compositor died.
  if (!software_render && time(NULL) - display_started < 5) {
    software_render = 1;
  }
  // Respawn the compositor + squishy (our terminal emulator). squishy execs
  // $SHELL (=/bin/cjsh) inside the pty it opens. zonule uses libseat's builtin
  // backend (no seatd daemon), so there's no socket to clean up here.
  spawn_display();
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

// Set (or clear) the Mesa software-render env according to software_render.
// Applied right before each compositor launch so the fallback in sigreap takes
// effect on the very next respawn.
static void apply_render_env(void) {
  if (software_render) {
    setenv("LIBGL_ALWAYS_SOFTWARE", "1", 1);
    setenv("GALLIUM_DRIVER", "llvmpipe", 1);
  } else {
    unsetenv("LIBGL_ALWAYS_SOFTWARE");
    unsetenv("GALLIUM_DRIVER");
  }
}

// Launch the compositor, recording the launch time so sigreap can distinguish a
// fast GL/EGL-init failure (→ software fallback) from an ordinary later exit.
static void spawn_display(void) {
  apply_render_env();
  display_started = time(NULL);
  spawn((char *[]){"/bin/display", "-s", "/bin/squishy", NULL});
}

// True if the kernel command line (/proc/cmdline) contains `token`.
static int cmdline_has(const char *token) {
  int fd = open("/proc/cmdline", O_RDONLY);
  if (fd == -1) return 0;
  char buf[1024];
  ssize_t n = read(fd, buf, sizeof(buf) - 1);
  close(fd);
  if (n <= 0) return 0;
  buf[n] = '\0';
  return strstr(buf, token) != NULL;
}
