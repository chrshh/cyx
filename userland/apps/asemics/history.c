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

/* responsible for adding entry to history and resizing if needed */
void historyCheckpoint(Row *curr_row) {
    if (E.history->length >= E.history->capacity) { historyResize(); }

    E.history->length = E.history->curr_idx;

    Row    row_copy    = deepCopyRow(curr_row);
    Cursor cursor_copy = deepCopyCursor(&E.cursor);

    HistoryItem new_entry;
    new_entry.cursor = cursor_copy;
    new_entry.row    = row_copy;

    /* add entry and align length + current index */
    E.history->records[E.history->length] = new_entry;
    E.history->length++;
    E.history->curr_idx = E.history->length;
    addDbgLog("RENDER: %s | LEN: %d", E.history->records[E.history->length - 1].row.render,
              E.history->length);
};

void historyUndo() {
    /* base case of no more history to see */
    if (E.history->curr_idx < 1) {
        editorSetStatusMsg("Already at earliest history");
        return;
    }

    /* stash current state once so redo always has a target */
    if (E.history->length == E.history->curr_idx) {
        if (E.history->length >= E.history->capacity) historyResize();
        Row *cur                                     = &E.buffer.rows[E.cursor.y];
        E.history->records[E.history->length].row    = deepCopyRow(cur);
        E.history->records[E.history->length].cursor = deepCopyCursor(&E.cursor);
        E.history->length++;
    }

    HistoryItem *entry = &E.history->records[E.history->curr_idx - 1];
    editorFreeRow(&E.buffer.rows[entry->cursor.y]);
    E.buffer.rows[entry->cursor.y] = deepCopyRow(&entry->row);
    E.cursor                       = entry->cursor;
    E.history->curr_idx--;
};

void historyRedo() {
    if (E.history->curr_idx + 1 >= E.history->length) {
        editorSetStatusMsg("Already at latest history");
        return;
    }

    HistoryItem *entry = &E.history->records[E.history->curr_idx + 1];
    editorFreeRow(&E.buffer.rows[entry->cursor.y]);
    E.buffer.rows[entry->cursor.y] = deepCopyRow(&entry->row);
    E.cursor                       = entry->cursor;
    E.history->curr_idx++;
};

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

/* doubles capacity when called && reallocs records mem */
void historyResize() {
    E.history->capacity *= 2;
    E.history->records = realloc(E.history->records, sizeof(HistoryItem) * E.history->capacity);
}
