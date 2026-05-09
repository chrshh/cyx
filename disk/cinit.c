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
  mount("proc", "/proc", "proc", 0, NULL);
  mount("sysfs", "/sys", "sys", 0, NULL);
  mount("devtmpfs", "/dev", "dev", 0, NULL);
  mount("/dev/vdb", "/home", "ext4", 0, NULL);
  chdir("/home");

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
  spawn((char *[]){"/bin/cjsh", NULL});

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
  // Respawn our boy
  spawn((char *[]){"/bin/cjsh", NULL});
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
    ioctl(STDIN_FILENO, TIOCSCTTY, 0);
    execvp(argv[0], argv);
    perror("execvp");
    _exit(1);
  case -1:
    perror("fork");
  }
}
