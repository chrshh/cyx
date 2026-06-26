#ifndef ASEMICS_H
#define ASEMICS_H

#include <stddef.h>
#include <stdio.h>
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

#define STATUS_BAR_RESERVE 2
#define LINE_NUM_RESERVE   8

#define SCROLL_OFF 5

/* commands */
#define SAVE  (1 << 0)
#define QUIT  (1 << 1)
#define FORCE (1 << 2)

/* colors */
#define DARK_GRAY  "\x1b[90m"
#define BLUE       "\x1b[94m"
#define GREEN      "\x1b[38;2;165;214;255m"
#define KEYWORD1   "\x1b[31m"
#define KEYWORD2   "\x1b[31m"
#define DEF_COLOR  "\x1b[39m"
#define COMMENT    "\x1b[90m"
#define MLCOMMENT  "\x1b[90m"
#define MATCH      "\x1b[35m"
#define OPERATOR   BLUE
#define FULL_RESET "\x1b[0m"
#define RESET_FG   "\x1b[39m"

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

typedef struct Row {
    int            idx;    /* this row's position in the buffer */
    int            size;   /* length of `chars` (raw line bytes) */
    int            rsize;  /* length of `render` (after tab expansion) */
    char          *chars;  /* raw line text as stored in the file */
    char          *render; /* line as drawn on screen (tabs expanded etc.) */
    unsigned char *hl;     /* per-cell highlight codes, same length as `render` */
    int hl_open_comment;   /* true if this row ends inside an unclosed multi-line comment */
} Row;

typedef struct {
    int x;  /* cursor column in the buffer (logical char index, ignores tabs) */
    int y;  /* cursor row in the buffer (line index) */
    int rx; /* cursor column on screen (`x` with tabs expanded) */
} Cursor;

typedef struct {
    int height;  /* number of visible rows on screen */
    int width;   /* number of visible columns on screen */
    int row_off; /* topmost visible buffer row (vertical scroll offset) */
    int col_off; /* leftmost visible column (horizontal scroll offset) */
} Viewport;

typedef struct {
    Row  *rows;     /* dynamic array of rows — the document contents */
    int   num_rows; /* number of rows currently in `rows` */
    bool  dirty;    /* true if there are unsaved changes */
    char *filename; /* path to the open file (NULL when unnamed) */
} Buffer;

typedef struct {
    char   msg[80];     /* transient status message shown at the bottom */
    time_t msg_time;    /* when `msg` was set, used to time it out */
    char   cmdline[80]; /* text being typed in command mode (`:w`, `:q`, ...) */
} StatusBar;

typedef struct HistoryItem {
    Cursor cursor;
    Row    row;
} HistoryItem;

typedef struct History {
    HistoryItem *records;
    unsigned int capacity;
    unsigned int length;
    unsigned int curr_idx;
} History;

typedef struct Editor {
    EditorMode     mode;     /* current modal state (normal/insert/command/visual) */
    Cursor         cursor;   /* where the cursor is */
    Viewport       viewport; /* what slice of the buffer is visible */
    Buffer         buffer;   /* the document being edited */
    StatusBar      ui;       /* bottom-bar state: status message + command line */
    EditorSyntax  *syntax;   /* active syntax-highlight rules (NULL = none) */
    History       *history;
    struct termios orig_term; /* termios snapshot from before raw mode, restored on exit */
} Editor;

/* write buffer */
typedef struct {
    char *data;
    int   len;
} WriteBuf;

/* position struct for motions */
typedef struct {
    int x;
    int y;
} Pos;

extern Editor  E;
extern History H;
extern FILE   *dbg;

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
void handleLeaderKey(void);

/* editor.c */
void     refreshScreen(void);
void     clearScreen(void);
void     drawRows(WriteBuf *wb);
void     initEditor(Editor *e, History *h);
WriteBuf writeBufInit(void);
void     writeBufFree(WriteBuf *wb);
void     writeBufAppend(WriteBuf *wb, const char *s, int len);
void     moveCursor(int key);
void     editorOpen(char *filename);
void     editorScroll(void);
void     updateCursorShape(WriteBuf *wb);
void     editorQuit(bool force);
void     editorSave(void);
int      editorCreateFile(char *filename);
char    *editorPrompt(char *prompt, void (*callback)(char *, int));
void     welcomeScreen(WriteBuf *wb);
void     drawLineNums(WriteBuf *wb, int filerow);

/* rows.c */
void  editorInsertRow(int pos, char *s, size_t len);
void  editorInsertNewLine();
void  editorUpdateRow(Row *row);
int   editorRowXtoRx(Row *row, int x);
int   editorRowRxToX(Row *row, int rx);
void  editorRowInsertChar(Row *row, int pos, int c);
void  editorInsertChar(int c);
char *editorRowsToStr(int *buflen);
void  editorRowDelChar(Row *row, int pos);
void  editorDelChar(void);
void  editorFreeRow(Row *row);
void  editorDelRow(int pos);
void  editorRowAppendString(Row *row, char *s, size_t len);

/* statbar.c */
void  editorDrawStatusBar(WriteBuf *wb);
char *getModeStr(void);
void  editorSetStatusMsg(const char *fmt, ...);
void  editorDrawMsgBar(WriteBuf *wb);

/* commands.c */
int  parseCommands(char *cmd, int len);
void execCommands(void);
void commandInsertChar(int c);
void commandDelChar(void);
void editorDrawCmdline(WriteBuf *wb);

/* syntax_hl.c */
void  editorUpdateSyntax(Row *row);
char *editorSyntaxToColor(int hl);
int   is_separator(int c);
int   is_operator(int c);
void  editorSetSyntaxHighlight(void);

/* history.c */
History *initHistory();
void     historyResize();
void     historyCheckpoint(Row *row);
void     historyUndo();
void     historyRedo();
Row      deepCopyRow(Row *og_row);
Cursor   deepCopyCursor(Cursor *og_cursor);

/*
 *
 * motionfns.c
 *
 */

/* words */
Pos motionWordForward(void);
Pos motionWordForwardBig(void);
Pos motionWordEnd(void);
Pos motionWordEndBig(void);
Pos motionWordBackwards(void);
Pos motionWordBackwardsBig(void);

/* lines */
Pos motionLineLastChar(void);

/* actions */
Pos actionInsertLineBelowCursor(void);
Pos actionInsertLineAboveCursor(void);

/* search.c */
void editorFind(void);
void editorFindCallback(char *query, int key);

/* dbg.c */
void initDbg();
void addDbgLog(const char *fmt, ...) __attribute__((format(printf, 1, 2)));
;

enum editorKey { BACKSPACE = 127, ARROW_LEFT = 1000, ARROW_RIGHT, ARROW_UP, ARROW_DOWN };

/* highlighting rules */
#define HL_HIGHLIGHT_NUMBERS (1 << 0)
#define HL_HIGHLIGHT_STRINGS (1 << 1)

#define HLDB_ENTRIES (sizeof(HLDB) / sizeof(HLDB[0]))

#endif
