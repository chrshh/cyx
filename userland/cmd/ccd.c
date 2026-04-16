#include <core/panic.h>
#include <fs/fs.h>

int main(int argc, char *argv[]) {
  if (argc > 2) {
    panic("usage: cd <file>");
  }
  String path = StringFromLiteral(argv[1]);
  if (!path.chars) {
    path.chars[0] = '.';
    path.chars[1] = '.';
    path.chars[3] = '\0';
  }
  ChangeDir(path);
  return 0;
}
