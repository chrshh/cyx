#include <dirent.h>
#include <sys/stat.h>

#ifndef BUILTINS_H
#define BUILTINS_H

#define LS_LONG (1 << 0) // -l
#define LS_ALL (1 << 1)  // -a

typedef int (*BuildinFn)(char **argv);

typedef struct {
  char *name;
  BuildinFn fn;
} Builtin;

int ccd(int argc, char *argv[]);
int ex(int argc, char *argv[]);

#endif
