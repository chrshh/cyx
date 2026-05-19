#include <unistd.h>
#include <termios.h>

#include "textr.h"

EditorConfig cfg;

int main(int argc, char **argv) {
  enableRawMode();
  initEditor(&cfg);
  if (argc >= 2) {
    editorOpen(argv[1]);
  }

  editorSetStatusMsg(":q -> QUIT\t :w -> SAVE");

  while (1) {
    refreshScreen();
    processKey();
  }

  return 0;
}
