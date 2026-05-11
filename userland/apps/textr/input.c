#include <errno.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <stdbool.h>

#include "textr.h"

int readKey(void) {
  int nread;
  char c;

  nread = read(STDIN_FILENO, &c, 1);
  if (nread == -1 && errno != EAGAIN) die("read");

  /* ESC sequences */
  if (c == '\x1b') {
    char seq[3];

    if (read(STDIN_FILENO, &seq[0], 1) != 1) return '\x1b';
    if (read(STDIN_FILENO, &seq[1], 1) != 1) return '\x1b';

    if (seq[0] == '[') {
      switch (seq[1]) {
      case 'A':
        return ARROW_UP;
      case 'B':
        return ARROW_DOWN;
      case 'C':
        return ARROW_RIGHT;
      case 'D':
        return ARROW_LEFT;
      }
    }

    return '\x1b';
  } else {
    return c;
  }
}

void processKey(void) {
  int c = readKey();

  switch (c) {

  /* QUIT  */
  case CTRL_KEY('q'):
    clearScreen();
    exit(0);
    break;

  /* COMPLEX MOTIONS */
  case 'G': {
    int scrl_down = cfg.numrows;
    while (scrl_down--) {
      moveCursor('j');
    }
  } break;

  // TODO: Find words
  case '$': {
    erow *row = (cfg.y >= cfg.numrows) ? NULL : &cfg.er[cfg.y];
    cfg.x = strlen(row->chars) - 1;
  } break;

  case '^':
    cfg.x = 0;
    break;

  /* BASIC MOTIONS */
  case 'h':
  case 'j':
  case 'k':
  case 'l':
  case ARROW_UP:
  case ARROW_DOWN:
  case ARROW_LEFT:
  case ARROW_RIGHT:
    moveCursor(c);
    break;
  }
}
