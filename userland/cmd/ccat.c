#include <core/panic.h>
#include <fs/fs.h>
#include <stdio.h>

int main(int argc, char *argv[]) {
  if (argc < 2) {
    panic("usage: cat <file>");
  }
  String path = StrFromChar(argv[1]);
  String file = ReadFile(path);
  printf("%s", file.chars);
  return 0;
}
