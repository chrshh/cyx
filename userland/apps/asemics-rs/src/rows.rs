use crate::consts::TAB_STOP;
use crate::editor::Editor;

#[derive(Clone)]
pub struct Row {
    pub idx: i32,             /* this row's position in the buffer */
    pub chars: Vec<u8>,       /* raw line text as stored in the file */
    pub render: Vec<u8>,      /* line as drawn on screen (tabs expanded etc.) */
    pub hl: Vec<u8>,          /* per-cell highlight codes, same length as `render` */
    pub hl_open_comment: bool, /* true if this row ends inside an unclosed multi-line comment */
}

impl Row {
    /* the C code tracks size/rsize alongside the buffers; here they are always
     * the buffer lengths */
    pub fn size(&self) -> i32 {
        self.chars.len() as i32
    }

    pub fn rsize(&self) -> i32 {
        self.render.len() as i32
    }

    pub fn x_to_rx(&self, x: i32) -> i32 {
        let mut rx = 0;
        for j in 0..x {
            if (j as usize) < self.chars.len() && self.chars[j as usize] == b'\t' {
                rx += (TAB_STOP - 1) - (rx % TAB_STOP);
            }
            rx += 1;
        }
        rx
    }

    pub fn rx_to_x(&self, rx: i32) -> i32 {
        let mut cur_rx = 0;
        let mut cx = 0;
        while cx < self.size() {
            if self.chars[cx as usize] == b'\t' {
                cur_rx += (TAB_STOP - 1) - (cur_rx % TAB_STOP);
            }
            cur_rx += 1;

            if cur_rx > rx {
                return cx;
            }
            cx += 1;
        }
        cx
    }
}

impl Editor {
    pub fn insert_row(&mut self, pos: i32, s: &[u8]) {
        if pos < 0 || pos > self.buffer.num_rows {
            return;
        }

        let row = Row {
            idx: pos,
            chars: s.to_vec(),
            render: Vec::new(),
            hl: Vec::new(),
            hl_open_comment: false,
        };
        self.buffer.rows.insert(pos as usize, row);
        for j in (pos + 1) as usize..self.buffer.rows.len() {
            self.buffer.rows[j].idx += 1;
        }

        self.update_row(pos);

        self.buffer.num_rows += 1;
    }

    pub fn insert_new_line(&mut self) {
        if self.cursor.x == 0 {
            self.insert_row(self.cursor.y, b"");
        } else {
            let y = self.cursor.y as usize;
            let x = self.cursor.x as usize;
            let rest = self.buffer.rows[y].chars[x..].to_vec();
            self.insert_row(self.cursor.y + 1, &rest);
            self.buffer.rows[y].chars.truncate(x);
            self.update_row(self.cursor.y);
        }
        self.cursor.y += 1;
        self.cursor.x = 0;
    }

    pub fn update_row(&mut self, idx: i32) {
        let row = &mut self.buffer.rows[idx as usize];

        let mut render = Vec::with_capacity(row.chars.len());
        for &c in &row.chars {
            if c == b'\t' {
                render.push(b' ');
                while render.len() % TAB_STOP as usize != 0 {
                    render.push(b' ');
                }
            } else {
                render.push(c);
            }
        }
        row.render = render;

        self.update_syntax(idx);
    }

    pub fn row_insert_char(&mut self, idx: i32, pos: i32, c: i32) {
        let row = &mut self.buffer.rows[idx as usize];
        let mut pos = pos;
        if pos < 0 || pos > row.size() {
            pos = row.size();
        }
        row.chars.insert(pos as usize, c as u8);
        self.update_row(idx);
    }

    pub fn insert_char(&mut self, c: i32) {
        if self.cursor.y == self.buffer.num_rows {
            self.insert_row(self.buffer.num_rows, b"");
        }
        self.row_insert_char(self.cursor.y, self.cursor.x, c);
        self.cursor.x += 1;
        self.buffer.dirty = true;
    }

    pub fn del_row(&mut self, pos: i32) {
        if pos < 0 || pos >= self.buffer.num_rows {
            return;
        }
        self.buffer.rows.remove(pos as usize);
        for j in pos as usize..self.buffer.rows.len() {
            self.buffer.rows[j].idx -= 1;
        }
        self.buffer.num_rows -= 1;
        self.buffer.dirty = true;
    }

    pub fn row_del_char(&mut self, idx: i32, pos: i32) {
        let row = &mut self.buffer.rows[idx as usize];
        if pos < 0 || pos >= row.size() {
            return;
        }
        row.chars.remove(pos as usize);
        self.update_row(idx);
        self.buffer.dirty = true;
    }

    pub fn del_char(&mut self) {
        if self.cursor.y == self.buffer.num_rows {
            return;
        }
        if self.cursor.x == 0 && self.cursor.y == 0 {
            return;
        }

        if self.cursor.x > 0 {
            self.row_del_char(self.cursor.y, self.cursor.x - 1);
            self.cursor.x -= 1;
        } else {
            let y = self.cursor.y as usize;
            self.cursor.x = self.buffer.rows[y - 1].size();
            let s = self.buffer.rows[y].chars.clone();
            self.row_append_string(self.cursor.y - 1, &s);
            self.del_row(self.cursor.y);
            self.cursor.y -= 1;
        }
    }

    pub fn row_append_string(&mut self, idx: i32, s: &[u8]) {
        self.buffer.rows[idx as usize].chars.extend_from_slice(s);
        self.update_row(idx);
        self.buffer.dirty = true;
    }
}
