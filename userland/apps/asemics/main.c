#include <unistd.h>
#include <termios.h>
#include "asemics.h"

Editor  E;
History H;
FILE   *dbg;

int main(int argc, char **argv) {
    enableRawMode();

    /*
     * read debug logs
     * "tail -f /tmp/asemics.log"
     */
    initDbg();

    initEditor(&E, &H);
    if (argc >= 2) { editorOpen(argv[1]); }

    // editorSetStatusMsg(":q -> QUIT\t :w -> SAVE");

    while (1) {
        refreshScreen();
        processKey();
    }

    return 0;
}
