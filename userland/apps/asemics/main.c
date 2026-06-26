#include <unistd.h>
#include <termios.h>

#include "asemics.h"

Editor E;

int main(int argc, char **argv) {
  enableRawMode();
  initEditor(&E);
  if (argc >= 2) {
    editorOpen(argv[1]);
  }

  // editorSetStatusMsg(":q -> QUIT\t :w -> SAVE");

  while (1) {
    refreshScreen();
    processKey();
  }

  return 0;
}
