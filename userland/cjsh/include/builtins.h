#include <dirent.h>
#include <stddef.h>
#include <sys/stat.h>

#ifndef BUILTINS_H
#define BUILTINS_H

#define LS_LONG (1 << 0) // -l
#define LS_ALL (1 << 1)  // -a

typedef int (*BuiltinFn)(char **argv);

typedef struct {
  char *name;
  BuiltinFn fn;
} Builtin;

extern Builtin builtins[];
extern size_t builtins_len;

int cd(char **argv);
int ext(char **argv);
int expt(char **argv);
int pwd(char **argv);

int geten(char **argv);

#endif
