#ifndef TEXTR_H
#define TEXTR_H

#include <stddef.h>
#include <termios.h>
#include <time.h>
#include <stdbool.h>

/* keys */
#define CTRL_KEY(k) ((k) & 0x1f)
#define ENTER       '\r'

#define LEADER ' '

/* cursor & screen */
#define CURSOR_TL    "\x1b[H"
#define CURSOR_HIDE  "\x1b[?25l"
#define CURSOR_SHOW  "\x1b[?25h"
#define CURSOR_BLOCK "\x1b[2 q"
#define CURSOR_BAR   "\x1b[6 q"
#define CURSOR_RESET "\x1b[0 q"

#define SCREEN_CLEAR      "\x1b[2J"
#define SCREEN_CLEAR_LINE "\x1b[K"

#define TAB_STOP 8

/* commands */
#define SAVE  (1 << 0)
#define QUIT  (1 << 1)
#define FORCE (1 << 2)

/* colors */
#define BLUE      "\x1b[94m"
#define GREEN     "\x1b[38;2;165;214;255m"
#define KEYWORD1  "\x1b[31m"
#define KEYWORD2  "\x1b[31m"
#define DEF_COLOR "\x1b[39m"
#define COMMENT   "\x1b[90m"
#define MLCOMMENT "\x1b[90m"
#define MATCH     "\x1b[35m"
#define OPERATOR  BLUE

/* global state */
typedef enum { MODE_NORMAL, MODE_INSERT, MODE_COMMAND, MODE_VISUAL } EditorMode;

typedef enum {
    HL_NORMAL = 0,
    HL_NUMBER,
    HL_STRING,
    HL_COMMENT,
    HL_MLCOMMENT,
    HL_KEYWORD1,
    HL_KEYWORD2,
    HL_OPERATOR,
    HL_MATCH
} EditorHighlight;

typedef struct {
    char  *filetype;
    char **filematch;
    char **keywords;
    char  *operators;
    char  *singleline_comment_start;
    char  *multiline_comment_start;
    char  *multiline_comment_end;
    int    flags;
} EditorSyntax;

typedef struct erow {
    int            idx;
    int            size;
    int            rsize;
    char          *chars;
    char          *render;
    unsigned char *hl;
    int            hl_open_comment;
} erow;

typedef struct EditorConfig {
    EditorMode     mode;
    int            rows;
    int            cols;
    int            rowoff;
    int            coloff;
    int            x;
    int            y;
    int            rx;
    int            numrows;
    erow          *er;
    bool           dirty;
    char          *filename;
    char           statusmsg[80];
    time_t         statusmsg_time;
    char           cmdline[80];
    EditorSyntax  *syntax;
    struct termios orig_term;
} EditorConfig;

/* write buffer */
typedef struct {
    char *b;
    int   len;
} wBuf;

/* position struct for motions */
typedef struct {
    int x;
    int y;
} Pos;

extern EditorConfig cfg;

/* terminal.c */
void die(const char *s);
void disableRawMode(void);
void enableRawMode(void);
int  getWindowSize(int *rows, int *cols);
int  getCursorPosition(int *rows, int *cols);

/* input.c */
int  readKey(void);
void processKey(void);
void handleInsertModeKey(int c);
void handleVisualModeKey(int c);
void handleCommandModeKey(int c);
void handleNormalModeKey(int c);
void handleLeaderKeyBind(void);

/* editor.c */
void  refreshScreen(void);
void  clearScreen(void);
void  drawRows(wBuf *wb);
void  initEditor(EditorConfig *cfg);
wBuf  initWBuf(void);
void  wBFree(wBuf *wb);
void  wBufAppend(wBuf *wb, const char *s, int len);
void  moveCursor(int key);
void  editorOpen(char *filename);
void  editorScroll(void);
void  updateCursorType(wBuf *wb);
void  editorQuit(bool force);
void  editorSave(void);
int   editorTouchFile(char *filename);
char *editorPrompt(char *prompt, void (*callback)(char *, int));

/* rows.c */
void  editorInsertRow(int pos, char *s, size_t len);
void  editorInsertNewLine();
void  editorUpdateRow(erow *er);
int   editorRowXtoRx(erow *er, int x);
int   editorRowRxToX(erow *er, int rx);
void  editorRowInsertChar(erow *er, int pos, int c);
void  editorInsertChar(int c);
char *editorRowsToStr(int *buflen);
void  editorRowDelChar(erow *er, int pos);
void  editorDelChar(void);
void  editorFreeRow(erow *er);
void  editorDelRow(int pos);
void  editorRowAppendString(erow *er, char *s, size_t len);

/* statbar.c */
void  editorDrawStatusBar(wBuf *wb);
char *getModeStr(void);
void  editorSetStatusMsg(const char *fmt, ...);
void  editorDrawMsgBar(wBuf *wb);

/* commands.c */
int  parseCommands(char *cmd, int len);
void execCommands(void);
void commandInsertChar(int c);
void commandDelChar(void);
void editorDrawCmdline(wBuf *wb);

/* syntax_hl.c */
void  editorUpdateSyntax(erow *row);
char *editorSyntaxToColor(int hl);
int   is_separator(int c);
int   is_operator(int c);
void  editorSetSyntaxHighlight(void);

/* motionfns.c */
Pos motionWordForward(void);
Pos motionWordEnd(void);
Pos motionWordBackwards(void);
Pos actionInsertLineBelowCursor(void);
Pos actionInsertLineAboveCursor(void);

/* search.c */
void editorFind(void);
void editorFindCallback(char *query, int key);

enum editorKey { BACKSPACE = 127, ARROW_LEFT = 1000, ARROW_RIGHT, ARROW_UP, ARROW_DOWN };

/* highlighting rules */
#define HL_HIGHLIGHT_NUMBERS (1 << 0)
#define HL_HIGHLIGHT_STRINGS (1 << 1)

#define HLDB_ENTRIES (sizeof(HLDB) / sizeof(HLDB[0]))

#endif
