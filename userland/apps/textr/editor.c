#include <ctype.h>
#include <errno.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <fcntl.h>
#include "textr.h"

void initEditor(Editor *e) {
    e->mode             = MODE_NORMAL;
    e->cursor.x         = 0;
    e->cursor.y         = 0;
    e->cursor.rx        = 0;
    e->viewport.row_off = 0;
    e->viewport.col_off = 0;
    e->buffer.num_rows  = 0;
    e->buffer.rows      = NULL;
    e->buffer.dirty     = false;
    e->buffer.filename  = NULL;
    e->ui.msg[0]        = '\0';
    e->ui.msg_time      = 0;
    e->ui.cmdline[0]    = '\0';
    e->syntax           = NULL;
    if (getWindowSize(&e->viewport.height, &e->viewport.width) == -1) die("getWindowSize");
    e->viewport.height -= 2; // these rows are reserved for the status bar(s) at the bottom
}

void repositionCursorTL(WriteBuf *wb) {
    writeBufAppend(wb, CURSOR_TL, 3);
}

/* draw fn for entire editor */
void drawRows(WriteBuf *wb) {
    int y;
    for (y = 0; y < E.viewport.height - 1; y++) {
        int filerow = y + E.viewport.row_off;
        if (filerow >= E.buffer.num_rows) {
            if (E.buffer.num_rows == 0 && y == E.viewport.height / 3) {
                /* welcome screen rendered when no file is selected */
                welcomeScreen(wb);
            } else {
                writeBufAppend(wb, "~", 1);
            }
        } else {
            int len = E.buffer.rows[filerow].rsize - E.viewport.col_off;
            if (len < 0) len = 0;
            if (len > E.viewport.width) len = E.viewport.width;
            char          *c          = &E.buffer.rows[filerow].render[E.viewport.col_off];
            unsigned char *hl         = &E.buffer.rows[filerow].hl[E.viewport.col_off];
            char          *curr_color = NULL;
            int            j;
            for (j = 0; j < len; j++) {
                if (iscntrl(c[j])) {
                    char sym = (c[j] <= 26) ? '@' + c[j] : '?';
                    writeBufAppend(wb, "\x1b[7m", 4);
                    writeBufAppend(wb, &sym, 1);
                    writeBufAppend(wb, "\x1b[m", 3);
                    if (curr_color != NULL) { writeBufAppend(wb, curr_color, strlen(curr_color)); }
                } else if (hl[j] == HL_NORMAL) {
                    if (curr_color != NULL) {
                        writeBufAppend(wb, DEF_COLOR, 5);
                        curr_color = NULL;
                    }
                    writeBufAppend(wb, &c[j], 1);
                } else {
                    char *color = editorSyntaxToColor(hl[j]);
                    if (color != curr_color) {
                        curr_color = color;
                        writeBufAppend(wb, color, strlen(color));
                    }
                    writeBufAppend(wb, &c[j], 1);
                }
            }
            writeBufAppend(wb, DEF_COLOR, 5);
        }

        writeBufAppend(wb, "\x1b[K", 3);
        writeBufAppend(wb, "\r\n", 2);
    }
}

void welcomeScreen(WriteBuf *wb) {
    char welcome_title[50];
    char welcome_desc1[45];
    char welcome_desc2[60];

    int titlelen = snprintf(welcome_title, sizeof(welcome_title), "asemics -- 0.1");
    int desclen1 =
        snprintf(welcome_desc1, sizeof(welcome_desc1), "Asemic - mark-making that resembles text");
    int desclen2 = snprintf(welcome_desc2, sizeof(welcome_desc2),
                            " or handwriting but carries no specific literal meaning.");

    if (titlelen > E.viewport.width) titlelen = E.viewport.width;
    if (desclen1 > E.viewport.width) desclen1 = E.viewport.width;
    if (desclen2 > E.viewport.width) desclen2 = E.viewport.width;

    int title_padding = (E.viewport.width - titlelen) / 2;
    int desc_padding1 = (E.viewport.width - desclen1) / 2;
    int desc_padding2 = (E.viewport.width - desclen2) / 2;

    if (title_padding) {
        writeBufAppend(wb, "~", 1);
        title_padding--;
    }

    /* title */
    while (title_padding--)
        writeBufAppend(wb, " ", 1);
    writeBufAppend(wb, welcome_title, titlelen);
    writeBufAppend(wb, "\r\n\n", 3);

    /* desc 1 */
    while (desc_padding1--)
        writeBufAppend(wb, " ", 1);
    writeBufAppend(wb, welcome_desc1, desclen1);
    writeBufAppend(wb, "\r\n", 2);

    /* desc 2 */
    while (desc_padding2--)
        writeBufAppend(wb, " ", 1);
    writeBufAppend(wb, welcome_desc2, desclen2);
}

void updateCursorShape(WriteBuf *wb) {
    switch (E.mode) {
    case MODE_NORMAL: writeBufAppend(wb, CURSOR_BLOCK, 5); break;
    case MODE_COMMAND: writeBufAppend(wb, CURSOR_HIDE, 6); break;
    case MODE_INSERT: writeBufAppend(wb, CURSOR_BAR, 5); break;
    case MODE_VISUAL: writeBufAppend(wb, CURSOR_BLOCK, 5); break;
    default: return;
    }
}

void refreshScreen(void) {
    editorScroll();
    WriteBuf wb = writeBufInit();

    writeBufAppend(&wb, CURSOR_HIDE, 6);
    writeBufAppend(&wb, SCREEN_CLEAR, 4);
    repositionCursorTL(&wb);
    drawRows(&wb);

    char buf[32];
    int  n;

    n = snprintf(buf, sizeof(buf), "\x1b[%d;1H", E.viewport.height + 1);
    writeBufAppend(&wb, buf, n);
    editorDrawStatusBar(&wb);

    n = snprintf(buf, sizeof(buf), "\x1b[%d;1H", E.viewport.height + 2);
    writeBufAppend(&wb, buf, n);
    if (E.ui.cmdline[0] != '\0') {
        editorDrawCmdline(&wb);
    } else {
        editorDrawMsgBar(&wb);
    }

    n = snprintf(buf, sizeof(buf), "\x1b[%d;%dH", (E.cursor.y - E.viewport.row_off) + 1,
                 (E.cursor.rx - E.viewport.col_off) + 1);
    writeBufAppend(&wb, buf, n);

    /* Enabled cursor and render cursor based on EDITOR MODE */
    writeBufAppend(&wb, CURSOR_SHOW, 6);
    updateCursorShape(&wb);
    write(STDOUT_FILENO, wb.data, wb.len);
    writeBufFree(&wb);
}

void clearScreen(void) {
    write(STDOUT_FILENO, SCREEN_CLEAR, 4);
    write(STDOUT_FILENO, CURSOR_TL, 3);
}

WriteBuf writeBufInit() {
    WriteBuf wb;
    wb.data   = NULL;
    wb.len = 0;
    return wb;
}

void writeBufAppend(WriteBuf *wb, const char *s, int len) {
    char *new = realloc(wb->data, wb->len + len);

    if (new == NULL) return;
    memcpy(&new[wb->len], s, len);
    wb->data = new;
    wb->len += len;
}

void writeBufFree(WriteBuf *wb) {
    free(wb->data);
}

void moveCursor(int key) {
    Row *row = (E.cursor.y >= E.buffer.num_rows) ? NULL : &E.buffer.rows[E.cursor.y];
    switch (key) {
    case 'h':
    case ARROW_LEFT:
        if (E.cursor.x != 0) { E.cursor.x--; }
        break;

    case 'j':
    case ARROW_DOWN:
        if (E.cursor.y < E.buffer.num_rows) { E.cursor.y++; }
        break;

    case 'k':
    case ARROW_UP:
        if (E.cursor.y != 0) { E.cursor.y--; }
        break;

    case 'l':
    case ARROW_RIGHT:
        if (row && E.cursor.x < row->size - 1) { E.cursor.x++; }
        break;
    }

    row        = (E.cursor.y >= E.buffer.num_rows) ? NULL : &E.buffer.rows[E.cursor.y];
    int rowlen = row ? row->size : 0;
    if (E.cursor.x > rowlen) { E.cursor.x = rowlen; }
}

int editorCreateFile(char *filename) {
    int fd = open(filename, O_CREAT | O_WRONLY, 0644);
    if (fd == -1) {
        perror(filename);
        return 1;
    }

    close(fd);
    return 0;
}

void editorOpen(char *filename) {
    free(E.buffer.filename);
    E.buffer.filename = strdup(filename);

    editorSetSyntaxHighlight();

    FILE *fp = fopen(filename, "r");
    if (!fp) {
        int ok = editorCreateFile(filename);
        if (ok != 0) { die("open & create"); }
        fp = fopen(filename, "r");
    }

    char   *line    = NULL;
    size_t  linecap = 0;
    ssize_t linelen;
    while ((linelen = getline(&line, &linecap, fp)) != -1) {
        while (linelen > 0 && (line[linelen - 1] == '\n' || line[linelen - 1] == '\r'))
            linelen--;

        editorInsertRow(E.buffer.num_rows, line, linelen);
    }
    free(line);
    fclose(fp);
    E.buffer.dirty = false;
}

void editorScroll() {
    E.cursor.rx = 0;
    if (E.cursor.y < E.buffer.num_rows) { E.cursor.rx = editorRowXtoRx(&E.buffer.rows[E.cursor.y], E.cursor.x); }

    if (E.cursor.y < E.viewport.row_off) { E.viewport.row_off = E.cursor.y; }
    if (E.cursor.y >= E.viewport.row_off + E.viewport.height) { E.viewport.row_off = E.cursor.y - E.viewport.height + 1; }
    if (E.cursor.rx < E.viewport.col_off) { E.viewport.col_off = E.cursor.rx; }
    if (E.cursor.rx >= E.viewport.col_off + E.viewport.width) { E.viewport.col_off = E.cursor.rx - E.viewport.width + 1; }
}

char *editorRowsToStr(int *buflen) {
    int totallen = 0;
    int j;

    for (j = 0; j < E.buffer.num_rows; j++)
        totallen += E.buffer.rows[j].size + 1;
    *buflen = totallen;

    char *buf = malloc(totallen);
    char *p   = buf;

    for (j = 0; j < E.buffer.num_rows; j++) {
        memcpy(p, E.buffer.rows[j].chars, E.buffer.rows[j].size);
        p += E.buffer.rows[j].size;
        *p = '\n';
        p++;
    }

    return buf;
}

void editorSave() {
    if (E.buffer.filename == NULL) return;

    int   len;
    char *buf = editorRowsToStr(&len);

    int fd = open(E.buffer.filename, O_RDWR | O_CREAT, 0644);
    if (fd != -1) {
        if (ftruncate(fd, len) != -1) {
            if (write(fd, buf, len) == len) {
                close(fd);
                free(buf);
                E.buffer.dirty   = false;
                E.ui.cmdline[0]  = '\0';
                editorSetStatusMsg("%d: bytes written to disk", len);
                editorSetSyntaxHighlight();
                return;
            }
            editorSetSyntaxHighlight();
        }
        close(fd);
    }
    free(buf);
    editorSetStatusMsg("Failed to save. I/O error: %s", strerror(errno));
}

void editorQuit(bool force) {
    if (force) {
        clearScreen();
        exit(0);
    } else {
        if (E.buffer.dirty) {
            char buf[80];
            int  n = snprintf(buf, sizeof(buf),
                              "%s has unsaved changes. '!q' to quit without saving", E.buffer.filename);
            int  i = 0;
            E.ui.cmdline[0] = '\0';
            while (buf[i] != '\0') {
                commandInsertChar(buf[i]);
                i++;
            }
        } else {
            clearScreen();
            exit(0);
        }
    }
}

char *editorPrompt(char *prompt, void (*callback)(char *, int)) {
    size_t bufsize = 80;
    char  *buf     = malloc(bufsize);

    size_t buflen = 0;
    buf[0]        = '\0';

    while (1) {
        editorSetStatusMsg(prompt, buf);
        refreshScreen();

        int c = readKey();
        if (c == BACKSPACE) {
            if (buflen != 0) buf[--buflen] = '\0';
        } else if (c == '\x1b') {
            editorSetStatusMsg("");
            if (callback) callback(buf, c);
            free(buf);
            return NULL;
        } else if (c == '\r') {
            if (buflen != 0) {
                editorSetStatusMsg("");
                if (callback) callback(buf, c);
                return buf;
            }
        } else if (!iscntrl(c) && c < 128) {
            if (buflen == bufsize - 1) {
                bufsize *= 2;
                buf = realloc(buf, bufsize);
            }
            buf[buflen++] = c;
            buf[buflen]   = '\0';
        }

        if (callback) callback(buf, c);
    }
}
