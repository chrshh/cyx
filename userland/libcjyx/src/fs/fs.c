#include <core/ansi.h>
#include <core/memory.h>
#include <core/panic.h>
#include <dirent.h>
#include <fcntl.h>
#include <fs/fs.h>
#include <getopt.h>
#include <stdio.h>
#include <sys/stat.h>
#include <unistd.h>

/* read write execute
 [ownner][group][others]
   rwx     rwx    rwx
*/
// static char rwx[3] = "rwx"; // 7
// static char r_x[3] = "r-x"; // 6
// static char r__[3] = "r--"; // 5

int FileExists(String path) {
  struct stat sb;
  if (stat(path.chars, &sb) == 0) {
    if (S_ISREG(sb.st_mode)) {
      return 1;
    }
  }
  return 0;
}

int IsDir(String path) {
  struct stat sb;
  if (stat(path.chars, &sb) == 0) {
    if (S_ISDIR(sb.st_mode)) {
      return 1;
    }
  }
  return 0;
}

String ReadFile(String path) {
  int exists = FileExists(path);
  if (exists == 0) {
    panic("file does not exist");
  }

  String out;
  struct stat sb;
  stat(path.chars, &sb);
  size_t filesize = sb.st_size;
  out.chars = cmalloc(filesize + 1);
  out.capacity = filesize;

  int fd = open(path.chars, O_RDONLY);
  if (fd == -1) {
    panic("could not open file");
  }
  ssize_t bytesread = read(fd, out.chars, filesize);
  if (bytesread == -1) {
    panic("failed to read file");
  }
  close(fd);

  out.len = bytesread;
  out.chars[filesize] = '\0';
  return out;
}

void WriteFile(String path, String in) {
  size_t bytes = in.len;

  int fd = open(path.chars, O_WRONLY | O_CREAT | O_TRUNC, 0644);
  if (fd == -1) {
    panic("failed to open file");
  }
  ssize_t byteswritten = write(fd, in.chars, bytes);
  if (byteswritten == -1) {
    panic("failed to write to file");
  }
  close(fd);
}

void ChangeDir(String path) {
  if (chdir(path.chars) == -1) {
    return;
  }
}

DIR *OpenDir(char *path) {
  DIR *dirp = opendir(path);
  if (dirp == NULL) {
    perror("failed to open dir");
    panic("failed to open dir");
  }
  return dirp;
}

// Takes octal val (sb.st_mode & 0777) and formats
String FormatPermsOctal(int perm) {
  String permstr = NewStr();
  char str[4];

  sprintf(str, "%o", perm);
  for (int i = 0; i < 3; i++) {
    switch (str[i]) {
    case '7':
      StrAppend(&permstr, "rwx");
      break;
    case '6':
      StrAppend(&permstr, "rw-");
      break;
    case '5':
      StrAppend(&permstr, "r-x");
      break;
    case '4':
      StrAppend(&permstr, "r--");
      break;
    default:
      StrAppend(&permstr, "---");
      break;
    }
  }
  StrAppend(&permstr, "@");
  return permstr;
}
