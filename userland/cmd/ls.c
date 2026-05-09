#include <core/ansi.h>
#include <dirent.h>
#include <fs/fs.h>
#include <stdio.h>
#include <string.h>
#include <unistd.h>
#include "ls.h"

int ParseFlags(int argc, char *argv[]) {
  int flags = 0;
  int opts;

  while ((opts = getopt(argc, argv, "la")) != -1) {
    switch (opts) {
    case 'l':
      flags |= LS_LONG;
      break;
    case 'a':
      flags |= LS_ALL;
      break;
    }
  }

  return flags;
}

// Function used only for cls cmd
void PrintEntries(struct dirent *dp, struct stat *sb, int flags) {
  if (!(flags & LS_ALL) && dp->d_name[0] == '.') return;

  if (flags & LS_LONG) {
    String perms = FormatPermsOctal(sb->st_mode & 0777);
    const char *color = (dp->d_type == DT_DIR) ? CYAN : "";
    const char *reset = (dp->d_type == DT_DIR) ? RESET : "";
    printf("%-10s %8ld  %s%s%s\n", perms.chars, (long)sb->st_size, color, dp->d_name, reset);
    return;
  }

  if (dp->d_type == DT_DIR) {
    printf(CYAN "%s  " RESET, dp->d_name);
  } else {
    printf("%s  ", dp->d_name);
  }

  return;
}

int main(int argc, char *argv[]) {
  int flags = ParseFlags(argc, argv);
  DIR *dir = OpenDir(".");
  struct dirent *dp;
  struct stat sb;

  while ((dp = readdir(dir)) != NULL) {
    stat(dp->d_name, &sb);
    PrintEntries(dp, &sb, flags);
  }
  printf("\n");
  closedir(dir);
  return 0;
}
