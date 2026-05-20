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
        memcpy(cfg.er[saved_hl_line].hl, saved_hl, cfg.er[saved_hl_line].rsize);
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
    for (i = 0; i < cfg.numrows; i++) {
        current += direction;
        if (current == -1) current = cfg.numrows - 1;
        else if (current == cfg.numrows) current = 0;

        erow *row   = &cfg.er[current];
        char *match = strstr(row->render, query);
        if (match) {
            last_match = current;
            cfg.y      = current;
            cfg.x      = editorRowRxToX(row, match - row->render);
            cfg.rowoff = cfg.numrows;

            saved_hl_line = current;
            saved_hl      = malloc(row->rsize);
            memcpy(saved_hl, row->hl, row->rsize);

            memset(&row->hl[match - row->render], HL_MATCH, strlen(query));
            break;
        }
    }
}

void editorFind() {
    int saved_x      = cfg.x;
    int saved_y      = cfg.y;
    int saved_coloff = cfg.coloff;
    int saved_rowoff = cfg.rowoff;

    char *query = editorPrompt("/%s", editorFindCallback);
    if (query) {
        free(query);
    } else {
        cfg.x      = saved_x;
        cfg.y      = saved_y;
        cfg.coloff = saved_coloff;
        cfg.rowoff = saved_rowoff;
    }
}
