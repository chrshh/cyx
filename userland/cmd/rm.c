#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>
#include <core/warn.h>

int main(int argc, char **argv) {
  if (argc < 2) {
    warn("usage: rm <file>");
    return 1;
  }

  int fd = unlink(argv[1]);
  if (fd == -1) {
    perror(argv[1]);
    exit(127);
  }
  close(fd);
  return 0;
}
