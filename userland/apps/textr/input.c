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

    switch (cfg.mode) {
    case MODE_INSERT: handleInsertModeKey(c); break;
    case MODE_NORMAL: handleNormalModeKey(c); break;
    case MODE_COMMAND: handleCommandModeKey(c); break;
    case MODE_VISUAL: handleVisualModeKey(c); break;
    }
}

void handleNormalModeKey(int c) {
    /* grab current row cursor is on */
    erow *row = (cfg.y >= cfg.numrows) ? NULL : &cfg.er[cfg.y];

    switch (c) {
    /* QUIT  */
    case CTRL_KEY('q'): editorQuit(false); break;

    case CTRL_KEY('s'): editorSave(); break;

    case LEADER: handleLeaderKeyBind(); break;

    case '/': editorFind(); break;

    /* COMPLEX MOTIONS */
    case 'G': {
        int scrl_down = cfg.numrows;
        while (scrl_down--) {
            moveCursor('j');
        }
    } break;

    case 'g': {
        int d = readKey();
        switch (d) {
        case 'g': cfg.y = 0; break;
        default: break;
        }
    }

    case '$': {
        erow *row = (cfg.y >= cfg.numrows) ? NULL : &cfg.er[cfg.y];
        if (row && row->size > 0) { cfg.x = strlen(row->chars) - 1; }
    } break;

    case '^': cfg.x = 0; break;

    case 'i': cfg.mode = MODE_INSERT; break;

    case 'a':
        cfg.mode = MODE_INSERT;
        cfg.x++;
        break;

    case ':':
        cfg.mode       = MODE_COMMAND;
        cfg.cmdline[0] = ':';
        cfg.cmdline[1] = '\0';
        break;

    case 'v': cfg.mode = MODE_VISUAL; break;

    case 'w': {
        Pos target = motionWordForward();
        cfg.x      = target.x;
        cfg.y      = target.y;
        break;
    }
    // case 'e': motionWordEnd(row); break;
    // case 'b': motionWordBackwards(row); break;

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
        cfg.mode       = MODE_NORMAL;
        cfg.cmdline[0] = '\0';
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
        cfg.mode = MODE_NORMAL;
        cfg.x--;
        break;

    case ARROW_DOWN:
    case ARROW_UP:
    case ARROW_LEFT:
    case ARROW_RIGHT: moveCursor(c); break;

    default: editorInsertChar(c); break;
    }
    return;
}

void handleLeaderKeyBind(void) {
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
