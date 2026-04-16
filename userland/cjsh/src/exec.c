#include <parser.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <core/panic.h>
#include <sys/wait.h>
#include <builtins.h>

int execute(char *argc, char *argv[]) {
  pid_t pid;
  pid_t w;
  int wstatus;

  for (size_t i = 0; i < builtins_len; i++) {
    if (strcmp(builtins[i].name, argc) == 0) {
      builtins[i].fn(argv);
      return 0;
    }
  }

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
