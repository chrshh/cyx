#include "textr.h"
#include <stdarg.h>
#include <stdlib.h>
#include <string.h>

void editorFindCallback(char *query, int key) {
    static int last_match = -1;
    static int direction  = 1;

    static int   saved_hl_line;
    static char *saved_hl = NULL;

    if (saved_hl) {
        memcpy(E.buffer.rows[saved_hl_line].hl, saved_hl, E.buffer.rows[saved_hl_line].rsize);
        free(saved_hl);
        saved_hl = NULL;
    }

    if (key == '\r' || key == '\x1b') {
        last_match = -1;
        direction  = 1;
        return;
    } else if (key == ARROW_DOWN) {
        direction = 1;
        return;
    } else if (key == ARROW_UP) {
        direction = -1;
    } else {
        last_match = -1;
        direction  = 1;
    }

    if (last_match == -1) direction = 1;
    int current = last_match;

    int i;
    for (i = 0; i < E.buffer.num_rows; i++) {
        current += direction;
        if (current == -1) current = E.buffer.num_rows - 1;
        else if (current == E.buffer.num_rows) current = 0;

        Row  *row   = &E.buffer.rows[current];
        char *match = strstr(row->render, query);
        if (match) {
            last_match         = current;
            E.cursor.y         = current;
            E.cursor.x         = editorRowRxToX(row, match - row->render);
            E.viewport.row_off = E.buffer.num_rows;

            saved_hl_line = current;
            saved_hl      = malloc(row->rsize);
            memcpy(saved_hl, row->hl, row->rsize);

            memset(&row->hl[match - row->render], HL_MATCH, strlen(query));
            break;
        }
    }
}

void editorFind() {
    int saved_x      = E.cursor.x;
    int saved_y      = E.cursor.y;
    int saved_coloff = E.viewport.col_off;
    int saved_rowoff = E.viewport.row_off;

    char *query = editorPrompt("/%s", editorFindCallback);
    if (query) {
        free(query);
    } else {
        E.cursor.x         = saved_x;
        E.cursor.y         = saved_y;
        E.viewport.col_off = saved_coloff;
        E.viewport.row_off = saved_rowoff;
    }
}
