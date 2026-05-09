#include <fs/fs.h>
#include <stdio.h>
#include <core/warn.h>
#include <unistd.h>

int main(int argc, char *argv[]) {
  if (argc < 2) {
    char buf[4096];
    ssize_t n;
    while ((n = read(STDIN_FILENO, buf, sizeof buf)) > 0)
      write(STDOUT_FILENO, buf, n);
    return 0;
  }

  String path = StrFromChar(argv[1]);
  String file = ReadFile(path);
  printf("%s", file.chars);
  return 0;
}
