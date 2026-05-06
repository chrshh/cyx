#include <sys/types.h>
#include <sys/wait.h>
#include <sys/mount.h>
#include <sys/reboot.h>

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
  chdir("/");
  mount("proc", "/proc", 0, NULL);
  mount("sysfs", "/sys", 0, NULL);
  mount("devtmpfs", "/dev", 0, NULL);

  // Not sure if this goes here
  open("/dev/console", O_RDWR);

  sigfillset(&set);
  sigprocmask(SIG_BLOCK, &set, NULL);
  spawn((char *[]){"/bin/cjsh"});

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

static void sigreap(void) {
  while (waitpid(-1, NULL, WNOHANG) > 0);
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
    execvp(argv[0], argv);
    perror("execvp");
    _exit(1);
  case -1:
    perror("fork");
  }
}
