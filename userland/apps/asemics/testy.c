#include <unistd.h>
#include <termios.h>



ookeid dookie poop ppppo
#include "asemics.h"
this is testy
Editor E;
last time testy broke;

kjj
this i


s the testy line;
 line;

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
okokokokok
