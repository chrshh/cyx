#ifndef TEXTR_H
#define TEXTR_H

#include <stddef.h>
#include <termios.h>

/* keys */
#define CTRL_KEY(k) ((k) & 0x1f)

/* cursor & screen */
#define CURSOR_TL "\x1b[H"
#define CURSOR_HIDE "\x1b[?25l"
#define CURSOR_SHOW "\x1b[?25h"

#define SCREEN_CLEAR "\x1b[2J"
#define SCREEN_CLEAR_LINE "\x1b[K"

/* global state */
typedef enum {
  MODE_NORMAL,
  MODE_INSERT,
  MODE_COMMAND,
  MODE_VISUAL
} EditorMode;

typedef struct erow {
  int size;
  char *chars;
} erow;

typedef struct EditorConfig {
  EditorMode mode;
  int rows;
  int cols;
  int rowoff;
  int coloff;
  int x;
  int y;
  int numrows;
  erow *er;
  struct termios orig_term;
} EditorConfig;

/* write buffer */
typedef struct {
  char *b;
  int len;
} wBuf;

extern EditorConfig cfg;

/* terminal.c */
void die(const char *s);
void disableRawMode(void);
void enableRawMode(void);
int getWindowSize(int *rows, int *cols);
int getCursorPosition(int *rows, int *cols);

/* input.c */
int readKey(void);
void processKey(void);

/* editor.c */
void refreshScreen(void);
void clearScreen(void);
void drawRows(wBuf *wb);
void initEditor(EditorConfig *cfg);
wBuf initWBuf(void);
void wBFree(wBuf *wb);
void wBufAppend(wBuf *wb, const char *s, int len);
void moveCursor(int key);
void editorOpen(char *filename);
void editorScroll(void);

/* rows.c */
void editorAppendRow(char *s, size_t len);

enum editorKey {
  BACKSPACE = 127,
  ARROW_LEFT = 1000,
  ARROW_RIGHT,
  ARROW_UP,
  ARROW_DOWN
};

#endif
