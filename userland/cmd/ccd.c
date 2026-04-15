#include <core/panic.h>
#include <fs/fs.h>

int main(int argc, char *argv[]) {
  if (argc < 2) {
    panic("usage: cd <file>");
  }
  String path = StringFromLiteral(argv[1]);
  ChangeDir(path);
  return 0;
}
