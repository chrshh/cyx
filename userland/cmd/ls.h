#pragma once
#ifndef LS_H
#define LS_H

#include <dirent.h>
#include <sys/stat.h>

#define LS_LONG (1 << 0) // -l
#define LS_ALL (1 << 1)  // -a

int ParseFlags(int argc, char *argv[]);
void PrintEntries(struct dirent *dp, struct stat *sb, int flags);

#endif
