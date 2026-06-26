#include "asemics.h"
#include <stdlib.h>
#include <string.h>

History *initHistory() {
    History *h  = malloc(sizeof(History));
    h->capacity = 512;
    h->length   = 0;
    h->curr_idx = h->length;
    h->records  = malloc(sizeof(HistoryItem) * h->capacity);
    return h;
}

/* responsible for adding entry to history */
void historyCheckpoint() {
    /* grab current row cursor is on */
    Row *curr_row = &E.buffer.rows[E.cursor.y];

    Row    row_copy    = deepCopyRow(curr_row);
    Cursor cursor_copy = deepCopyCursor(&E.cursor);

    HistoryItem new_entry;
    new_entry.cursor = cursor_copy;
    new_entry.row    = row_copy;

    E.history->records[E.history->length] = new_entry;
    E.history->length++;
    addDbgLog("RENDER: %s | LEN: %d", E.history->records->row.render, E.history->length);
};

void historyUndo() {};

void historyRedo() {};

/* row struct contains char* types that need to get deep copied */
Row deepCopyRow(Row *og_row) {
    Row row;
    /* shallow copies */
    row.idx             = og_row->idx;
    row.size            = og_row->size;
    row.rsize           = og_row->rsize;
    row.hl_open_comment = og_row->hl_open_comment;
    /* allocations for strings */
    row.chars  = malloc(strlen(og_row->chars) + 1);
    row.render = malloc(strlen(og_row->render) + 1);
    row.hl     = malloc(og_row->rsize);
    /* deep copies */
    strcpy(row.chars, og_row->chars);
    strcpy(row.render, og_row->render);
    memcpy(row.hl, og_row->hl, row.rsize);

    return row;
}

Cursor deepCopyCursor(Cursor *og_cursor) {
    Cursor cursor;
    cursor.rx = og_cursor->rx;
    cursor.x  = og_cursor->x;
    cursor.y  = og_cursor->y;
    return cursor;
}
