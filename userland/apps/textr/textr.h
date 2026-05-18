#ifndef TEXTR_H
#define TEXTR_H

#include <stddef.h>
#include <termios.h>
#include <time.h>
#include <stdbool.h>

/* keys */
#define CTRL_KEY(k) ((k) & 0x1f)

#define LEADER " "

/* cursor & screen */
#define CURSOR_TL "\x1b[H"
#define CURSOR_HIDE "\x1b[?25l"
#define CURSOR_SHOW "\x1b[?25h"

#define SCREEN_CLEAR "\x1b[2J"
#define SCREEN_CLEAR_LINE "\x1b[K"

#define TAB_STOP 8

/* global state */
typedef enum {
  MODE_NORMAL,
  MODE_INSERT,
  MODE_COMMAND,
  MODE_VISUAL
} EditorMode;

typedef struct erow {
  int size;
  int rsize;
  char *chars;
  char *render;
} erow;

typedef struct EditorConfig {
  EditorMode mode;
  int rows;
  int cols;
  int rowoff;
  int coloff;
  int x;
  int y;
  int rx;
  int numrows;
  erow *er;
  bool dirty;
  char *filename;
  char statusmsg[8];
  time_t statusmsg_time;
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
void handleInsertModeKey(int c);
void handleVisualModeKey(int c);
void handleCommandModeKey(int c);
void handleNormalModeKey(int c);

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
void editorInsertRow(int pos, char *s, size_t len);
void editorInsertNewLine();
void editorUpdateRow(erow *er);
int editorRowXtoRx(erow *er, int x);
void editorRowInsertChar(erow *er, int pos, int c);
void editorInsertChar(int c);
char *editorRowsToStr(int *buflen);
void editorSave();
void editorRowDelChar(erow *er, int pos);
void editorDelChar(void);
void editorFreeRow(erow *er);
void editorDelRow(int pos);
void editorRowAppendString(erow *er, char *s, size_t len);

/* statbar.c */
void editorDrawStatusBar(wBuf *wb);
char *getModeStr(void);
void editorSetStatusMsg(const char *fmt, ...);
void editorDrawMsgBar(wBuf *wb);

enum editorKey {
  BACKSPACE = 127,
  ARROW_LEFT = 1000,
  ARROW_RIGHT,
  ARROW_UP,
  ARROW_DOWN
};

#endif
