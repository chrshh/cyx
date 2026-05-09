#include <core/warn.h>
#include <fs/fs.h>
#include <fcntl.h>
#include <stdio.h>
#include <unistd.h>

/*
 * @brief Creates a file
 * @note Can currently only create one file at a time;
 */
int main(int argc, char **argv) {
  if (argc < 2) {
    warn("usage: touch <file>");
    return 1;
  }

  if (FileExistsChar(argv[1])) {
    return 0;
  }

  int fd = open(argv[1], O_CREAT | O_WRONLY);
  if (fd == -1) {
    perror(argv[1]);
    return 1;
  }

  close(fd);
  return 0;
}
