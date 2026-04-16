#include <parser.h>
#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>
#include <core/panic.h>
#include <sys/wait.h>

int execute(char *argc, char *argv[]) {
  pid_t pid;
  pid_t w;
  int wstatus;

  pid = fork();
  if (pid == 0) {
    execvp(argc, argv);
    printf("cjsh: command not found\n");
    exit(1);
  } else {
    w = waitpid(pid, &wstatus, WUNTRACED | WCONTINUED);
    if (w == -1) {
      panic("wait fart");
    }
  }

  return 0;
}
