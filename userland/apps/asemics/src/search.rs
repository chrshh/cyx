use crate::consts::{ENTER, ESC};
use crate::editor::Editor;
use crate::input::{ARROW_DOWN, ARROW_UP};
use crate::syntax::HL_MATCH;

/* strstr over bytes; like strstr, an empty needle matches at offset 0 */
fn find_sub(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    if needle.is_empty() {
        return Some(0);
    }
    haystack.windows(needle.len()).position(|w| w == needle)
}

impl Editor {
    pub fn find(&mut self) {
        let saved_x = self.cursor.x;
        let saved_y = self.cursor.y;
        let saved_coloff = self.viewport.col_off;
        let saved_rowoff = self.viewport.row_off;

        /* statics in the C callback; they live for one find session here, but
         * the C code resets them on every ENTER/ESC anyway */
        let mut last_match: i32 = -1;
        let mut direction: i32 = 1;
        let mut saved_hl_line: i32 = 0;
        let mut saved_hl: Option<Vec<u8>> = None;

        let query = self.prompt("/%s", |e, query, key| {
            if let Some(hl) = saved_hl.take() {
                e.buffer.rows[saved_hl_line as usize].hl = hl;
            }

            if key == ENTER || key == ESC {
                last_match = -1;
                direction = 1;
                return;
            } else if key == ARROW_DOWN {
                direction = 1;
                return;
            } else if key == ARROW_UP {
                direction = -1;
            } else {
                last_match = -1;
                direction = 1;
            }

            if last_match == -1 {
                direction = 1;
            }
            let mut current = last_match;

            for _ in 0..e.buffer.num_rows {
                current += direction;
                if current == -1 {
                    current = e.buffer.num_rows - 1;
                } else if current == e.buffer.num_rows {
                    current = 0;
                }

                let row_idx = current as usize;
                if let Some(m) = find_sub(&e.buffer.rows[row_idx].render, query) {
                    last_match = current;
                    e.cursor.y = current;
                    e.cursor.x = e.buffer.rows[row_idx].rx_to_x(m as i32);
                    e.viewport.row_off = e.buffer.num_rows;

                    saved_hl_line = current;
                    saved_hl = Some(e.buffer.rows[row_idx].hl.clone());

                    let qlen = query.len();
                    e.buffer.rows[row_idx].hl[m..m + qlen].fill(HL_MATCH);
                    break;
                }
            }
        });

        if query.is_none() {
            self.cursor.x = saved_x;
            self.cursor.y = saved_y;
            self.viewport.col_off = saved_coloff;
            self.viewport.row_off = saved_rowoff;
        }
    }
}
