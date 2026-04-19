#include "common.h"
#include <builtins.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <limits.h>
#include <unistd.h>
#include <core/ansi.h>
#include <fs/fs.h>
#include <core/panic.h>

/* cd */
int cd(char **argv) {
  String path;
  if (argv[1] != NULL) {
    path = StringFromLiteral(argv[1]);
  }
  if (path.chars == NULL) {
    return (0);
  }
  ChangeDir(path);
  GetShPrompt();
  return 0;
}

/* pwd */
int pwd(char **argv) {
  (void)argv;
  char buf[PATH_MAX];
  char *cwd = getcwd(buf, PATH_MAX);
  String wd;
  wd = StringFromLiteral(cwd);
  printf("%s\n", wd.chars);
  return 0;
}

int expt(char **argv) {
  if (strlen(*argv) < 2) {
    printf("not enough arguments for assignment");
    return -1;
  }
  char *key = argv[1];
  char *val = argv[2];
  if (!key || !val) {
    return -1;
  }
  int success = setenv(key, val, 0);
  if (success != 0) {
    printf("failed to set env");
    return -1;
  }
  return 0;
}

/* exit */
int ext(char **argv) {
  (void)argv;
  if (getpid() == 1) {
    printf("cjsh: cannot exit PID 1\n");
    return 0;
  } else {
    exit(0);
  }
}

int geten(char **argv) {
  char *env = argv[1];
  char *res = getenv(env);
  if (*res == -1) {
    res = "NOT FOUND";
    return -1;
  }
  printf("ENV: %s\n", res);
  return 0;
}

Builtin builtins[] = {
    {"cd", cd}, {"pwd", pwd}, {"expt", expt}, {"ext", ext}, {"env", geten},
};
size_t builtins_len = sizeof(builtins) / sizeof(builtins[0]);
