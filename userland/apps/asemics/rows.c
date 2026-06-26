#include <stdlib.h>
#include <string.h>

#include "asemics.h"

void editorInsertRow(int pos, char *s, size_t len) {
    if (pos < 0 || pos > E.buffer.num_rows) return;

    E.buffer.rows = realloc(E.buffer.rows, sizeof(Row) * (E.buffer.num_rows + 1));
    memmove(&E.buffer.rows[pos + 1], &E.buffer.rows[pos], sizeof(Row) * (E.buffer.num_rows - pos));
    for (int j = pos + 1; j <= E.buffer.num_rows; j++)
        E.buffer.rows[j].idx++;

    E.buffer.rows[pos].idx = pos;

    E.buffer.rows[pos].size  = len;
    E.buffer.rows[pos].chars = malloc(len + 1);
    memcpy(E.buffer.rows[pos].chars, s, len);
    E.buffer.rows[pos].chars[len] = '\0';

    E.buffer.rows[pos].rsize           = 0;
    E.buffer.rows[pos].render          = NULL;
    E.buffer.rows[pos].hl              = NULL;
    E.buffer.rows[pos].hl_open_comment = 0;
    editorUpdateRow(&E.buffer.rows[pos]);

    E.buffer.num_rows++;
}

void editorInsertNewLine(void) {
    if (E.cursor.x == 0) {
        editorInsertRow(E.cursor.y, "", 0);
    } else {
        Row *er = &E.buffer.rows[E.cursor.y];
        editorInsertRow(E.cursor.y + 1, &er->chars[E.cursor.x], er->size - E.cursor.x);
        er                  = &E.buffer.rows[E.cursor.y];
        er->size            = E.cursor.x;
        er->chars[er->size] = '\0';
        editorUpdateRow(er);
    }
    E.cursor.y++;
    E.cursor.x = 0;
}

void editorUpdateRow(Row *er) {
    int tabs = 0;
    int j;
    for (j = 0; j < er->size; j++)
        if (er->chars[j] == '\t') tabs++;

    free(er->render);
    er->render = malloc(er->size + tabs * (TAB_STOP - 1) + 1);

    int idx = 0;
    for (j = 0; j < er->size; j++) {
        if (er->chars[j] == '\t') {
            er->render[idx++] = ' ';
            while (idx % TAB_STOP != 0)
                er->render[idx++] = ' ';
        } else {
            er->render[idx++] = er->chars[j];
        }
    }

    er->render[idx] = '\0';
    er->rsize       = idx;

    editorUpdateSyntax(er);
}

int editorRowXtoRx(Row *er, int x) {
    int rx = 0;
    int j  = 0;

    for (j = 0; j < x; j++) {
        if (er->chars[j] == '\t') rx += (TAB_STOP - 1) - (rx % TAB_STOP);
        rx++;
    }
    return rx;
}

int editorRowRxToX(Row *row, int rx) {
    int cur_rx = 0;
    int cx;
    for (cx = 0; cx < row->size; cx++) {
        if (row->chars[cx] == '\t') cur_rx += (TAB_STOP - 1) - (cur_rx % TAB_STOP);
        cur_rx++;

        if (cur_rx > rx) return cx;
    }
    return cx;
}

void editorRowInsertChar(Row *er, int pos, int c) {
    if (pos < 0 || pos > er->size) pos = er->size;
    er->chars = realloc(er->chars, er->size + 2);
    memmove(&er->chars[pos + 1], &er->chars[pos], er->size - pos + 1);
    er->size++;
    er->chars[pos] = c;
    editorUpdateRow(er);
}

void editorInsertChar(int c) {
    if (E.cursor.y == E.buffer.num_rows) { editorInsertRow(E.buffer.num_rows, "", 0); }
    editorRowInsertChar(&E.buffer.rows[E.cursor.y], E.cursor.x, c);
    E.cursor.x++;
    E.buffer.dirty = true;
}

void editorFreeRow(Row *er) {
    free(er->render);
    free(er->chars);
    free(er->hl);
}

void editorDelRow(int pos) {
    if (pos < 0 || pos >= E.buffer.num_rows) return;
    editorFreeRow(&E.buffer.rows[pos]);
    memmove(&E.buffer.rows[pos], &E.buffer.rows[pos + 1],
            sizeof(Row) * (E.buffer.num_rows - pos - 1));
    for (int j = pos; j < E.buffer.num_rows - 1; j++)
        E.buffer.rows[j].idx--;
    E.buffer.num_rows--;
    E.buffer.dirty = true;
}

void editorRowDelChar(Row *er, int pos) {
    if (pos < 0 || pos >= er->size) return;
    memmove(&er->chars[pos], &er->chars[pos + 1], er->size - pos);
    er->size--;
    editorUpdateRow(er);
    E.buffer.dirty = true;
}

void editorDelChar(void) {
    if (E.cursor.y == E.buffer.num_rows) return;
    if (E.cursor.x == 0 && E.cursor.y == 0) return;

    Row *er = &E.buffer.rows[E.cursor.y];
    if (E.cursor.x > 0) {
        editorRowDelChar(er, E.cursor.x - 1);
        E.cursor.x--;
    } else {
        E.cursor.x = E.buffer.rows[E.cursor.y - 1].size;
        editorRowAppendString(&E.buffer.rows[E.cursor.y - 1], er->chars, er->size);
        editorDelRow(E.cursor.y);
        E.cursor.y--;
    }
}

void editorRowAppendString(Row *er, char *s, size_t len) {
    er->chars = realloc(er->chars, er->size + len + 1);
    memcpy(&er->chars[er->size], s, len);
    er->size += len;
    er->chars[er->size] = '\0';
    editorUpdateRow(er);
    E.buffer.dirty = true;
}
