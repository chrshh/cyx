#include <core/ansi.h>
#include <dirent.h>
#include <fs/fs.h>
#include <stdio.h>
#include <sys/stat.h>
#include <unistd.h>

#ifndef LS_H
#define LS_H

#define LS_LONG (1 << 0) // -l
#define LS_ALL (1 << 1)  // -a

#endif

int ParseFlags(int argc, char *argv[]) {
  int flags = 0;
  int opts;

  while ((opts = getopt(argc, argv, "la")) != -1) {
    switch (opts) {
    case 'l':
      flags |= LS_LONG;
    case 'a':
      flags |= LS_ALL;
    }
  }

  return flags;
}

// Function used only for cls cmd
void PrintEntries(struct dirent *dp, struct stat *sb, int flags) {
  if (!(flags & LS_ALL) && (dp->d_name[0] == '.' || dp->d_name[1] == '.'))
    return;

  if (flags & LS_LONG) {
    String perms = FormatPermsOctal(sb->st_mode & 0777);
    printf("%s %llo  ", perms.chars, sb->st_size);
  }

  if (dp->d_type == DT_DIR) {
    printf(CYAN "%s" RESET, dp->d_name);
  } else {
    printf("%s", dp->d_name);
  }

  printf("\n");
  return;
}

int main(int argc, char *argv[]) {
  int flags = ParseFlags(argc, argv);
  DIR *dir = OpenDir(".");
  struct dirent *dp;
  struct stat sb;
  if (flags & LS_LONG) {
    printf("AUTH  SIZE  FILE\n");
  }

  while ((dp = readdir(dir)) != NULL) {
    stat(dp->d_name, &sb);
    PrintEntries(dp, &sb, flags);
  }

  closedir(dir);
  return 0;
}
