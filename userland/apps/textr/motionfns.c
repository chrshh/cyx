#include "textr.h"
#include <string.h>

/*
 *
 * helper fns
 *
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

/*
 *
 * normal mode
 *
 */

/* -- w -- */
Pos motionWordForward(void) {
    Pos p = { E.cursor.x, E.cursor.y };
    if (p.y >= E.buffer.num_rows) return p;
    Row *row = &E.buffer.rows[p.y];

    if (p.x < row->size && charType(row->chars[p.x]) != TYPE_SPACE) {
        int start_type = charType(row->chars[p.x]);
        while (p.x < row->size && charType(row->chars[p.x]) == start_type) {
            p.x++;
        }
    }
    while (1) {
        if (p.x >= row->size) {
            if (p.y + 1 >= E.buffer.num_rows) {
                p.x = row->size;
                return p;
            }
            p.y++;
            p.x = 0;
            row = &E.buffer.rows[p.y];

            if (row->size == 0) return p;
            continue;
        }
        if (charType(row->chars[p.x]) != TYPE_SPACE) return p;
        p.x++;
    }
}

/* -- W -- */
Pos motionWordForwardBig(void) {
    Pos p = { E.cursor.x, E.cursor.y };
    if (p.y >= E.buffer.num_rows) return p;
    Row *row = &E.buffer.rows[p.y];
    if (p.x < row->size && charType(row->chars[p.x]) != TYPE_SPACE) {
        while (p.x < row->size && charType(row->chars[p.x]) != TYPE_SPACE) {
            p.x++;
        }
    }
    while (1) {
        if (p.x >= row->size) {
            if (p.y + 1 >= E.buffer.num_rows) {
                p.x = row->size;
                return p;
            }
            p.y++;
            p.x = 0;
            row = &E.buffer.rows[p.y];

            if (row->size == 0) return p;
            continue;
        }
        if (charType(row->chars[p.x]) != TYPE_SPACE) return p;
        p.x++;
    }
}

/* -- b -- */
Pos motionWordBackwards(void) {
    Pos p = { E.cursor.x, E.cursor.y };
    if (p.y >= E.buffer.num_rows) return p;
    Row *row = &E.buffer.rows[p.y];

    p.x--;
    if (p.x < 0) {
        if (p.y == 0) return p;
        p.y--;
        row = &E.buffer.rows[p.y];
        p.x = row->size ? row->size - 1 : 0;
        if (row->size == 0) return p;
    }

    while (1) {
        if (p.x < 0) {
            if (p.y == 0) {
                p.x = 0;
                return p;
            }
            p.y--;
            row = &E.buffer.rows[p.y];
            p.x = row->size > 0 ? row->size - 1 : 0;
            if (row->size == 0) return p;
            continue;
        }
        if (charType(row->chars[p.x]) != TYPE_SPACE) break;
        p.x--;
    }

    int run_type = charType(row->chars[p.x]);
    while (p.x > 0 && charType(row->chars[p.x - 1]) == run_type) {
        p.x--;
    }
    return p;
}

/* -- B -- */
Pos motionWordBackwardsBig(void) {
    Pos p = { E.cursor.x, E.cursor.y };
    if (p.y >= E.buffer.num_rows) return p;
    Row *row = &E.buffer.rows[p.y];

    p.x--;
    if (p.x < 0) {
        if (p.y == 0) return p;
        p.y--;
        row = &E.buffer.rows[p.y];
        p.x = row->size ? row->size - 1 : 0;
        if (row->size == 0) return p;
    }

    while (1) {
        if (p.x < 0) {
            if (p.y == 0) {
                p.x = 0;
                return p;
            }
            p.y--;
            row = &E.buffer.rows[p.y];
            p.x = row->size > 0 ? row->size - 1 : 0;
            if (row->size == 0) return p;
            continue;
        }
        if (charType(row->chars[p.x]) != TYPE_SPACE) break;
        p.x--;
    }

    while (p.x > 0 && charType(row->chars[p.x - 1]) != TYPE_SPACE) {
        p.x--;
    }
    return p;
}

/* -- e --  */
Pos motionWordEnd(void) {
    Pos p = { E.cursor.x, E.cursor.y };
    if (p.y >= E.buffer.num_rows) return p;
    Row *row = &E.buffer.rows[p.y];

    p.x++;
    if (p.x >= row->size) {
        if (p.y + 1 >= E.buffer.num_rows) {
            p.x = row->size > 0 ? row->size - 1 : 0;
            return p;
        }
        p.y++;
        p.x = 0;
        row = &E.buffer.rows[p.y];
    }

    /* skip whitespace & jump lines as needed */
    while (1) {
        if (p.x >= row->size) {
            if (p.y + 1 >= E.buffer.num_rows) {
                p.x = row->size > 0 ? row->size - 1 : 0;
                return p;
            }
            p.y++;
            p.x = 0;
            row = &E.buffer.rows[p.y];
            continue;
        }
        if (charType(row->chars[p.x]) != TYPE_SPACE) break;
        p.x++;
    }

    int run_type = charType(row->chars[p.x]);
    while (p.x < row->size && charType(row->chars[p.x]) == run_type) {
        p.x++;
    }
    p.x--;
    return p;
}

/* -- E -- */
Pos motionWordEndBig(void) {
    Pos p = { E.cursor.x, E.cursor.y };
    if (p.y >= E.buffer.num_rows) return p;
    Row *row = &E.buffer.rows[p.y];

    p.x++;
    if (p.x >= row->size) {
        if (p.y + 1 >= E.buffer.num_rows) {
            p.x = row->size > 0 ? row->size - 1 : 0;
            return p;
        }
        p.y++;
        p.x = 0;
        row = &E.buffer.rows[p.y];
    }

    while (1) {
        if (p.x >= row->size) {
            if (p.y + 1 >= E.buffer.num_rows) {
                p.x = row->size > 0 ? row->size - 1 : 0;
                return p;
            }
            p.y++;
            p.x = 0;
            row = &E.buffer.rows[p.y];
            continue;
        }
        if (charType(row->chars[p.x]) != TYPE_SPACE) break;
        p.x++;
    }
    while (p.x < row->size && charType(row->chars[p.x]) != TYPE_SPACE) {
        p.x++;
    }
    p.x--;
    return p;
}

/* -- $ -- */
Pos motionLineLastChar(void) {
    Pos   p   = { E.cursor.x, E.cursor.y };
    Row  *row = (p.y >= E.buffer.num_rows) ? NULL : &E.buffer.rows[p.y];
    if (row && row->size > 0) { p.x = strlen(row->chars) - 1; }
    return p;
}

/*
 *
 * insert mode
 *
 */

/* -- o -- */
Pos actionInsertLineBelowCursor(void) {
    Row *row = &E.buffer.rows[E.cursor.y];

    int indent = 0;
    while (indent < row->size && (row->chars[indent] == ' ' || row->chars[indent] == '\t')) {
        indent++;
    }

    E.cursor.x = row->size;
    editorInsertNewLine();

    for (int i = 0; i < indent; i++) {
        editorInsertChar(' ');
    }
    Pos pos = { E.cursor.x, E.cursor.y };
    return pos;
}

/* -- O -- */
Pos actionInsertLineAboveCursor(void) {
    Row *row = &E.buffer.rows[E.cursor.y];

    int indent = 0;
    while (indent < row->size && (row->chars[indent] == ' ' || row->chars[indent] == '\t')) {
        indent++;
    }

    E.cursor.x = 0;
    editorInsertNewLine();
    E.cursor.y--;

    for (int i = 0; i < indent; i++) {
        editorInsertChar(' ');
    }

    Pos pos = { E.cursor.x, E.cursor.y };
    return pos;
}
