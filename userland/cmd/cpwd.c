#include <core/panic.h>
#include <fs/fs.h>
#include <limits.h>
#include <stdio.h>
#include <unistd.h>

int main() {
  char buf[PATH_MAX];
  char *cwd = getcwd(buf, PATH_MAX);
  String wd;
  wd = StringFromLiteral(cwd);
  printf("%s\n", wd.chars);
  return 0;
}
