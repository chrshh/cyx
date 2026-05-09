#include <core/warn.h>
#include <stdio.h>
#include <stdlib.h>
#include <sys/stat.h>
#include <unistd.h>

int main(int argc, char **argv) {
  if (argc < 2) {
    warn("usage: mkdir <dir>");
    return 1;
  }

  int fd = mkdir(argv[1], 0755);
  if (fd == -1) {
    perror(argv[1]);
    exit(127);
  }
  close(fd);
  return 0;
}
