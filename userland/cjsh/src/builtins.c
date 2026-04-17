#include <builtins.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <limits.h>
#include <unistd.h>
#include <core/ansi.h>
#include <fs/fs.h>
#include <core/panic.h>

int ccd(char **argv) {
  String path;
  if (argv[1] != NULL) {
    path = StringFromLiteral(argv[1]);
  }
  if (path.chars == NULL) {
    return (0);
  }
  ChangeDir(path);
  return 0;
}

int cpwd(char **argv) {
  (void)argv;
  char buf[PATH_MAX];
  char *cwd = getcwd(buf, PATH_MAX);
  String wd;
  wd = StringFromLiteral(cwd);
  printf("%s\n", wd.chars);
  return 0;
}

int cexit(char **argv) {
  (void)argv;
  if (getpid() == 1) {
    printf("cjsh: cannot exit PID 1");
    return 0;
  } else {
    exit(0);
  }
}

Builtin builtins[] = {
    {"cd", ccd},
    {"pwd", cpwd},
    {"cexit", cexit},
};
size_t builtins_len = sizeof(builtins) / sizeof(builtins[0]);
