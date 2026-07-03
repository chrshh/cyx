use crate::editor::{Cursor, Editor};
use crate::rows::Row;

pub struct HistoryItem {
    pub cursor: Cursor,
    pub row: Row,
}

pub struct History {
    pub records: Vec<HistoryItem>,
    pub curr_idx: usize,
}

impl History {
    pub fn new() -> History {
        History {
            records: Vec::with_capacity(512),
            curr_idx: 0,
        }
    }
}

impl Editor {
    /* responsible for adding entry to history */
    pub fn history_checkpoint(&mut self) {
        /* drop any redo tail */
        self.history.records.truncate(self.history.curr_idx);

        let row_copy = self.buffer.rows[self.cursor.y as usize].clone();

        /* add entry and align length + current index */
        self.history.records.push(HistoryItem {
            cursor: self.cursor,
            row: row_copy,
        });
        self.history.curr_idx = self.history.records.len();

        let msg = format!(
            "RENDER: {} | LEN: {}",
            String::from_utf8_lossy(&self.history.records[self.history.curr_idx - 1].row.render),
            self.history.records.len()
        );
        self.add_dbg_log(&msg);
    }

    pub fn history_undo(&mut self) {
        /* base case of no more history to see */
        if self.history.curr_idx < 1 {
            self.set_status_msg("Already at earliest history".to_string());
            return;
        }

        /* stash current state once so redo always has a target */
        if self.history.records.len() == self.history.curr_idx {
            let cur = self.buffer.rows[self.cursor.y as usize].clone();
            self.history.records.push(HistoryItem {
                cursor: self.cursor,
                row: cur,
            });
        }

        let entry_cursor = self.history.records[self.history.curr_idx - 1].cursor;
        let entry_row = self.history.records[self.history.curr_idx - 1].row.clone();
        self.buffer.rows[entry_cursor.y as usize] = entry_row;
        self.cursor = entry_cursor;
        self.history.curr_idx -= 1;
    }

    pub fn history_redo(&mut self) {
        if self.history.curr_idx + 1 >= self.history.records.len() {
            self.set_status_msg("Already at latest history".to_string());
            return;
        }

        let entry_cursor = self.history.records[self.history.curr_idx + 1].cursor;
        let entry_row = self.history.records[self.history.curr_idx + 1].row.clone();
        self.buffer.rows[entry_cursor.y as usize] = entry_row;
        self.cursor = entry_cursor;
        self.history.curr_idx += 1;
    }
}
