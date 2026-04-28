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
    path = StrFromChar(argv[1]);
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
  wd = StrFromChar(cwd);
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
  int success = setenv(key, val, 1);
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

int rstenv(char **argv) {
  (void)argv;
  char *default_path = "/cjyx/cmd/bin:/usr/local/sbin:/usr/local/bin:/usr/"
                       "sbin:/usr/bin:/sbin:/bin";
  int res = setenv("PATH", default_path, 1);
  if (res != 0) {
    printf("cjsh: cannot reset PATH\n");
    return res;
  }
  return 0;
}

int prntenv(char **argv) {
  char *env = argv[1];
  char *res = getenv(env);
  if (res == NULL) {
    printf("cjsh: env variable not found\n");
    return -1;
  }
  printf("ENV: %s\n", res);
  return 0;
}

int bi(char **argv) {
  (void)argv;
  for (size_t i = 0; i < builtins_len; i++) {
    printf("%s\n", builtins[i].name);
  }
  return 0;
}

Builtin builtins[] = {{"cd", cd}, {"pwd", pwd}, {"expt", expt}, {"ext", ext}, {"prntenv", prntenv}, {"rstenv", rstenv}, {"bi", bi}};
size_t builtins_len = sizeof(builtins) / sizeof(builtins[0]);
