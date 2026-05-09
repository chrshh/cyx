#include <fs/fs.h>
#include <stdio.h>
#include <core/warn.h>

int main(int argc, char *argv[]) {
  if (argc < 2) {
    warn("usage: cat <file>");
  }
  String path = StrFromChar(argv[1]);
  String file = ReadFile(path);
  printf("%s\n", file.chars);
  return 0;
}
