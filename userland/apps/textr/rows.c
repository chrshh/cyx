#include <stdlib.h>
#include <string.h>

#include "textr.h"

void editorInsertRow(int pos, char *s, size_t len) {
    if (pos < 0 || pos > cfg.numrows) return;

    cfg.er = realloc(cfg.er, sizeof(erow) * (cfg.numrows + 1));
    memmove(&cfg.er[pos + 1], &cfg.er[pos], sizeof(erow) * (cfg.numrows - pos));
    for (int j = pos + 1; j <= cfg.numrows; j++)
        cfg.er[j].idx++;

    cfg.er[pos].idx = pos;

    cfg.er[pos].size  = len;
    cfg.er[pos].chars = malloc(len + 1);
    memcpy(cfg.er[pos].chars, s, len);
    cfg.er[pos].chars[len] = '\0';

    cfg.er[pos].rsize           = 0;
    cfg.er[pos].render          = NULL;
    cfg.er[pos].hl              = NULL;
    cfg.er[pos].hl_open_comment = 0;
    editorUpdateRow(&cfg.er[pos]);

    cfg.numrows++;
}

void editorInsertNewLine(void) {
    if (cfg.x == 0) {
        editorInsertRow(cfg.y, "", 0);
    } else {
        erow *er = &cfg.er[cfg.y];
        editorInsertRow(cfg.y + 1, &er->chars[cfg.x], er->size - cfg.x);
        er                  = &cfg.er[cfg.y];
        er->chars[er->size] = '\0';
        editorUpdateRow(er);
    }
    cfg.y++;
    cfg.x = 0;
}

void editorUpdateRow(erow *er) {
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

int editorRowXtoRx(erow *er, int x) {
    int rx = 0;
    int j  = 0;

    for (j = 0; j < x; j++) {
        if (er->chars[j] == '\t') rx += (TAB_STOP - 1) - (rx % TAB_STOP);
        rx++;
    }
    return rx;
}

int editorRowRxToX(erow *row, int rx) {
    int cur_rx = 0;
    int cx;
    for (cx = 0; cx < row->size; cx++) {
        if (row->chars[cx] == '\t') cur_rx += (TAB_STOP - 1) - (cur_rx % TAB_STOP);
        cur_rx++;

        if (cur_rx > rx) return cx;
    }
    return cx;
}

void editorRowInsertChar(erow *er, int pos, int c) {
    if (pos < 0 || pos > er->size) pos = er->size;
    er->chars = realloc(er->chars, er->size + 2);
    memmove(&er->chars[pos + 1], &er->chars[pos], er->size - pos + 1);
    er->size++;
    er->chars[pos] = c;
    editorUpdateRow(er);
}

void editorInsertChar(int c) {
    if (cfg.y == cfg.numrows) { editorInsertRow(cfg.numrows, "", 0); }
    editorRowInsertChar(&cfg.er[cfg.y], cfg.x, c);
    cfg.x++;
    cfg.dirty = true;
}

void editorFreeRow(erow *er) {
    free(er->render);
    free(er->chars);
    free(er->hl);
}

void editorDelRow(int pos) {
    if (pos < 0 || pos >= cfg.numrows) return;
    editorFreeRow(&cfg.er[pos]);
    memmove(&cfg.er[pos], &cfg.er[pos + 1], sizeof(erow) * (cfg.numrows - pos - 1));
    for (int j = pos; j < cfg.numrows - 1; j++)
        cfg.er[j].idx--;
    cfg.numrows--;
    cfg.dirty = true;
}

void editorRowDelChar(erow *er, int pos) {
    if (pos < 0 || pos >= er->size) return;
    memmove(&er->chars[pos], &er->chars[pos + 1], er->size - pos);
    er->size--;
    editorUpdateRow(er);
    cfg.dirty = true;
}

void editorDelChar(void) {
    if (cfg.y == cfg.numrows) return;
    if (cfg.x == 0 && cfg.y == 0) return;

    erow *er = &cfg.er[cfg.y];
    if (cfg.x > 0) {
        editorRowDelChar(er, cfg.x - 1);
        cfg.x--;
    } else {
        cfg.x = cfg.er[cfg.y - 1].size;
        editorRowAppendString(&cfg.er[cfg.y - 1], er->chars, er->size);
        editorDelRow(cfg.y);
        cfg.y--;
    }
}

void editorRowAppendString(erow *er, char *s, size_t len) {
    er->chars = realloc(er->chars, er->size + len + 1);
    memcpy(&er->chars[er->size], s, len);
    er->size += len;
    er->chars[er->size] = '\0';
    editorUpdateRow(er);
    cfg.dirty = true;
}
