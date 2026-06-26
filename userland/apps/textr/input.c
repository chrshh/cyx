#include <errno.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <stdbool.h>

#include "textr.h"

int readKey(void) {
    int  nread;
    char c;

    while ((nread = read(STDIN_FILENO, &c, 1)) != 1) {
        if (nread == -1 && errno != EAGAIN) die("read");
    }

    /* ESC sequences */
    if (c == '\x1b') {
        char seq[3];

        if (read(STDIN_FILENO, &seq[0], 1) != 1) return '\x1b';
        if (read(STDIN_FILENO, &seq[1], 1) != 1) return '\x1b';

        if (seq[0] == '[') {
            switch (seq[1]) {
            case 'A': return ARROW_UP;
            case 'B': return ARROW_DOWN;
            case 'C': return ARROW_RIGHT;
            case 'D': return ARROW_LEFT;
            }
        }

        return '\x1b';
    } else {
        return c;
    }
}

void processKey(void) {
    int c = readKey();

    switch (E.mode) {
    case MODE_INSERT: handleInsertModeKey(c); break;
    case MODE_NORMAL: handleNormalModeKey(c); break;
    case MODE_COMMAND: handleCommandModeKey(c); break;
    case MODE_VISUAL: handleVisualModeKey(c); break;
    }
}

void handleNormalModeKey(int c) {
    /* grab current row cursor is on */
    Row *row = (E.cursor.y >= E.buffer.num_rows) ? NULL : &E.buffer.rows[E.cursor.y];

    switch (c) {
    /* QUIT  */
    case CTRL_KEY('c'):
        clearScreen();
        exit(0);
        editorQuit(false);
        break;

    case CTRL_KEY('s'): editorSave(); break;

    case LEADER: handleLeaderKey(); break;

    case '/': editorFind(); break;

    case 'o': {
        Pos target  = actionInsertLineBelowCursor();
        E.cursor.x  = target.x;
        E.cursor.y  = target.y;
        E.mode      = MODE_INSERT;
        break;
    }

    case 'O': {
        Pos target  = actionInsertLineAboveCursor();
        E.cursor.x  = target.x;
        E.cursor.y  = target.y;
        E.mode      = MODE_INSERT;
        break;
    }

    /* COMPLEX MOTIONS */
    case 'G': {
        int scrl_down = E.buffer.num_rows;
        while (scrl_down--) {
            moveCursor('j');
        }
    } break;

    case 'g': {
        int d = readKey();
        switch (d) {
        case 'g': E.cursor.y = 0; break;
        default: break;
        }
    }

    case '$': {
        Pos target = motionLineLastChar();
        E.cursor.x = target.x;
        E.cursor.y = target.y;
        break;
        Row *row = (E.cursor.y >= E.buffer.num_rows) ? NULL : &E.buffer.rows[E.cursor.y];
        if (row && row->size > 0) { E.cursor.x = strlen(row->chars) - 1; }
    } break;

    case '^': E.cursor.x = 0; break;

    case 'i': E.mode = MODE_INSERT; break;

    case 'a':
        E.mode = MODE_INSERT;
        E.cursor.x++;
        break;

    case ':':
        E.mode          = MODE_COMMAND;
        E.ui.cmdline[0] = ':';
        E.ui.cmdline[1] = '\0';
        break;

    case 'v': E.mode = MODE_VISUAL; break;

    case 'w': {
        Pos target = motionWordForward();
        E.cursor.x = target.x;
        E.cursor.y = target.y;
        break;
    }

    case 'W': {
        Pos target = motionWordForwardBig();
        E.cursor.x = target.x;
        E.cursor.y = target.y;
        break;
    }

    case 'e': {
        Pos target = motionWordEnd();
        E.cursor.x = target.x;
        E.cursor.y = target.y;
        break;
    }

    case 'E': {
        Pos target = motionWordEndBig();
        E.cursor.x = target.x;
        E.cursor.y = target.y;
        break;
    }

    case 'b': {
        Pos target = motionWordBackwards();
        E.cursor.x = target.x;
        E.cursor.y = target.y;
        break;
    }

    case 'B': {
        Pos target = motionWordBackwardsBig();
        E.cursor.x = target.x;
        E.cursor.y = target.y;
    }

    /* BASIC MOTIONS */
    case 'h':
    case 'j':
    case 'k':
    case 'l':
    case ARROW_UP:
    case ARROW_DOWN:
    case ARROW_LEFT:
    case ARROW_RIGHT: moveCursor(c); break;
    }
}

void handleCommandModeKey(int c) {
    switch (c) {
        /* QUIT  */
    case CTRL_KEY('q'):
        clearScreen();
        exit(0);
        break;

    case ENTER: execCommands(); break;

    case BACKSPACE: commandDelChar(); break;

    case '\x1b':
        E.mode          = MODE_NORMAL;
        E.ui.cmdline[0] = '\0';
        break;
    default: commandInsertChar(c);
    }
}

void handleVisualModeKey(int c) {
    (void)c;
    return;
}

void handleInsertModeKey(int c) {
    switch (c) {
    /* enter key */
    case '\r': editorInsertNewLine(); break;

    /* QUIT  */
    case CTRL_KEY('q'): editorQuit(false); break;

    case BACKSPACE: editorDelChar(); break;

    case '\x1b':
        E.mode = MODE_NORMAL;
        E.cursor.x--;
        break;

    case ARROW_DOWN:
    case ARROW_UP:
    case ARROW_LEFT:
    case ARROW_RIGHT: moveCursor(c); break;

    default: editorInsertChar(c); break;
    }
    return;
}

void handleLeaderKey(void) {
    return;
}
//   int c = readKey();
//
//   switch (c) {
//   case '\x1b':
//     return;
//
//   case 'g':
//     int d = readKey();
//     switch (d) {
//         case 'g'
//       }
//   }
// }
