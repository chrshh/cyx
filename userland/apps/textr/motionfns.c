#include "textr.h"

/*
 * helper functions for determining how to move cursor for motions
 */
static int isWordChar(int c) {
    return (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || (c >= '0' && c <= '9') || c == '_';
}

enum { TYPE_WORD, TYPE_PUNCT, TYPE_SPACE };

static int charType(int c) {
    if (c == ' ' || c == '\t') return TYPE_SPACE;
    if (isWordChar(c)) return TYPE_WORD;
    return TYPE_PUNCT;
}

/* normal mode  */

/* -- w -- */
Pos motionWordForward(void) {
    Pos p = { cfg.x, cfg.y };
    if (p.y >= cfg.numrows) return p;
    erow *row = &cfg.er[p.y];

    if (p.x < row->size && charType(row->chars[p.x]) != TYPE_SPACE) {
        int start_type = charType(row->chars[p.x]);
        while (p.x < row->size && charType(row->chars[p.x]) == start_type) {
            p.x++;
        }
    }
    while (1) {
        if (p.x >= row->size) {
            if (p.y + 1 >= cfg.numrows) {
                p.x = row->size;
                return p;
            }
            p.y++;
            p.x = 0;
            row = &cfg.er[p.y];

            if (row->size == 0) return p;
            continue;
        }
        if (charType(row->chars[p.x]) != TYPE_SPACE) return p;
        p.x++;
    }
}

// Pos motionWordBackwards(void) {
// }
//
// Pos motionWordEnd(void) {
// }

/* insert mode */

/* -- o -- */
Pos actionInsertLineBelowCursor(void) {
    erow *row = &cfg.er[cfg.y];

    int indent = 0;
    while (indent < row->size && (row->chars[indent] == ' ' || row->chars[indent] == '\t')) {
        indent++;
    }

    cfg.x = row->size;
    editorInsertNewLine();

    for (int i = 0; i < indent; i++) {
        editorInsertChar(' ');
    }
    Pos pos = { cfg.x, cfg.y };
    return pos;
}

/* -- O -- */
Pos actionInsertLineAboveCursor(void) {
    erow *row = &cfg.er[cfg.y];

    int indent = 0;
    while (indent < row->size && (row->chars[indent] == ' ' || row->chars[indent] == '\t')) {
        indent++;
    }

    cfg.x = 0;
    editorInsertNewLine();
    cfg.y--;

    for (int i = 0; i < indent; i++) {
        editorInsertChar(' ');
    }

    Pos pos = { cfg.x, cfg.y };
    return pos;
}
