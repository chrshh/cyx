use crate::editor::{Editor, Pos};
use crate::rows::Row;

/*
 *
 * helper fns
 *
 */
fn is_word_char(c: u8) -> bool {
    c.is_ascii_alphanumeric() || c == b'_'
}

#[derive(Clone, Copy, PartialEq)]
enum CharType {
    Word,
    Punct,
    Space,
}

fn char_type(c: u8) -> CharType {
    if c == b' ' || c == b'\t' {
        CharType::Space
    } else if is_word_char(c) {
        CharType::Word
    } else {
        CharType::Punct
    }
}

/* the C code can index chars[x] with x == -1 (e.g. after ESC at column 0,
 * which is UB there); return 0, which types as Punct */
fn char_at(row: &Row, x: i32) -> u8 {
    if x >= 0 && (x as usize) < row.chars.len() {
        row.chars[x as usize]
    } else {
        0
    }
}

/*
 *
 * normal mode
 *
 */

impl Editor {
    /* -- w -- */
    pub fn motion_word_forward(&self) -> Pos {
        let mut p = Pos { x: self.cursor.x, y: self.cursor.y };
        if p.y >= self.buffer.num_rows {
            return p;
        }
        let mut row = &self.buffer.rows[p.y as usize];

        if p.x < row.size() && char_type(char_at(row, p.x)) != CharType::Space {
            let start_type = char_type(char_at(row, p.x));
            while p.x < row.size() && char_type(char_at(row, p.x)) == start_type {
                p.x += 1;
            }
        }
        loop {
            if p.x >= row.size() {
                if p.y + 1 >= self.buffer.num_rows {
                    p.x = row.size();
                    return p;
                }
                p.y += 1;
                p.x = 0;
                row = &self.buffer.rows[p.y as usize];

                if row.size() == 0 {
                    return p;
                }
                continue;
            }
            if char_type(char_at(row, p.x)) != CharType::Space {
                return p;
            }
            p.x += 1;
        }
    }

    /* -- W -- */
    pub fn motion_word_forward_big(&self) -> Pos {
        let mut p = Pos { x: self.cursor.x, y: self.cursor.y };
        if p.y >= self.buffer.num_rows {
            return p;
        }
        let mut row = &self.buffer.rows[p.y as usize];

        if p.x < row.size() && char_type(char_at(row, p.x)) != CharType::Space {
            while p.x < row.size() && char_type(char_at(row, p.x)) != CharType::Space {
                p.x += 1;
            }
        }
        loop {
            if p.x >= row.size() {
                if p.y + 1 >= self.buffer.num_rows {
                    p.x = row.size();
                    return p;
                }
                p.y += 1;
                p.x = 0;
                row = &self.buffer.rows[p.y as usize];

                if row.size() == 0 {
                    return p;
                }
                continue;
            }
            if char_type(char_at(row, p.x)) != CharType::Space {
                return p;
            }
            p.x += 1;
        }
    }

    /* -- b -- */
    pub fn motion_word_backwards(&self) -> Pos {
        let mut p = Pos { x: self.cursor.x, y: self.cursor.y };
        if p.y >= self.buffer.num_rows {
            return p;
        }
        let mut row = &self.buffer.rows[p.y as usize];

        p.x -= 1;
        if p.x < 0 {
            if p.y == 0 {
                return p;
            }
            p.y -= 1;
            row = &self.buffer.rows[p.y as usize];
            p.x = if row.size() != 0 { row.size() - 1 } else { 0 };
            if row.size() == 0 {
                return p;
            }
        }

        loop {
            if p.x < 0 {
                if p.y == 0 {
                    p.x = 0;
                    return p;
                }
                p.y -= 1;
                row = &self.buffer.rows[p.y as usize];
                p.x = if row.size() > 0 { row.size() - 1 } else { 0 };
                if row.size() == 0 {
                    return p;
                }
                continue;
            }
            if char_type(char_at(row, p.x)) != CharType::Space {
                break;
            }
            p.x -= 1;
        }

        let run_type = char_type(char_at(row, p.x));
        while p.x > 0 && char_type(char_at(row, p.x - 1)) == run_type {
            p.x -= 1;
        }
        p
    }

    /* -- B -- */
    pub fn motion_word_backwards_big(&self) -> Pos {
        let mut p = Pos { x: self.cursor.x, y: self.cursor.y };
        if p.y >= self.buffer.num_rows {
            return p;
        }
        let mut row = &self.buffer.rows[p.y as usize];

        p.x -= 1;
        if p.x < 0 {
            if p.y == 0 {
                return p;
            }
            p.y -= 1;
            row = &self.buffer.rows[p.y as usize];
            p.x = if row.size() != 0 { row.size() - 1 } else { 0 };
            if row.size() == 0 {
                return p;
            }
        }

        loop {
            if p.x < 0 {
                if p.y == 0 {
                    p.x = 0;
                    return p;
                }
                p.y -= 1;
                row = &self.buffer.rows[p.y as usize];
                p.x = if row.size() > 0 { row.size() - 1 } else { 0 };
                if row.size() == 0 {
                    return p;
                }
                continue;
            }
            if char_type(char_at(row, p.x)) != CharType::Space {
                break;
            }
            p.x -= 1;
        }

        while p.x > 0 && char_type(char_at(row, p.x - 1)) != CharType::Space {
            p.x -= 1;
        }
        p
    }

    /* -- e --  */
    pub fn motion_word_end(&self) -> Pos {
        let mut p = Pos { x: self.cursor.x, y: self.cursor.y };
        if p.y >= self.buffer.num_rows {
            return p;
        }
        let mut row = &self.buffer.rows[p.y as usize];

        p.x += 1;
        if p.x >= row.size() {
            if p.y + 1 >= self.buffer.num_rows {
                p.x = if row.size() > 0 { row.size() - 1 } else { 0 };
                return p;
            }
            p.y += 1;
            p.x = 0;
            row = &self.buffer.rows[p.y as usize];
        }

        /* skip whitespace & jump lines as needed */
        loop {
            if p.x >= row.size() {
                if p.y + 1 >= self.buffer.num_rows {
                    p.x = if row.size() > 0 { row.size() - 1 } else { 0 };
                    return p;
                }
                p.y += 1;
                p.x = 0;
                row = &self.buffer.rows[p.y as usize];
                continue;
            }
            if char_type(char_at(row, p.x)) != CharType::Space {
                break;
            }
            p.x += 1;
        }

        let run_type = char_type(char_at(row, p.x));
        while p.x < row.size() && char_type(char_at(row, p.x)) == run_type {
            p.x += 1;
        }
        p.x -= 1;
        p
    }

    /* -- E -- */
    pub fn motion_word_end_big(&self) -> Pos {
        let mut p = Pos { x: self.cursor.x, y: self.cursor.y };
        if p.y >= self.buffer.num_rows {
            return p;
        }
        let mut row = &self.buffer.rows[p.y as usize];

        p.x += 1;
        if p.x >= row.size() {
            if p.y + 1 >= self.buffer.num_rows {
                p.x = if row.size() > 0 { row.size() - 1 } else { 0 };
                return p;
            }
            p.y += 1;
            p.x = 0;
            row = &self.buffer.rows[p.y as usize];
        }

        loop {
            if p.x >= row.size() {
                if p.y + 1 >= self.buffer.num_rows {
                    p.x = if row.size() > 0 { row.size() - 1 } else { 0 };
                    return p;
                }
                p.y += 1;
                p.x = 0;
                row = &self.buffer.rows[p.y as usize];
                continue;
            }
            if char_type(char_at(row, p.x)) != CharType::Space {
                break;
            }
            p.x += 1;
        }
        while p.x < row.size() && char_type(char_at(row, p.x)) != CharType::Space {
            p.x += 1;
        }
        p.x -= 1;
        p
    }

    /* -- $ -- */
    pub fn motion_line_last_char(&self) -> Pos {
        let mut p = Pos { x: self.cursor.x, y: self.cursor.y };
        if p.y < self.buffer.num_rows {
            let row = &self.buffer.rows[p.y as usize];
            if row.size() > 0 {
                p.x = row.size() - 1;
            }
        }
        p
    }

    /*
     *
     * insert mode
     *
     */

    /* -- o -- */
    pub fn action_insert_line_below_cursor(&mut self) -> Pos {
        let row = &self.buffer.rows[self.cursor.y as usize];

        let mut indent = 0;
        while indent < row.size()
            && (char_at(row, indent) == b' ' || char_at(row, indent) == b'\t')
        {
            indent += 1;
        }
        let size = row.size();

        self.cursor.x = size;
        self.insert_new_line();

        for _ in 0..indent {
            self.insert_char(b' ' as i32);
        }
        Pos { x: self.cursor.x, y: self.cursor.y }
    }

    /* -- O -- */
    pub fn action_insert_line_above_cursor(&mut self) -> Pos {
        let row = &self.buffer.rows[self.cursor.y as usize];

        let mut indent = 0;
        while indent < row.size()
            && (char_at(row, indent) == b' ' || char_at(row, indent) == b'\t')
        {
            indent += 1;
        }

        self.cursor.x = 0;
        self.insert_new_line();
        self.cursor.y -= 1;

        for _ in 0..indent {
            self.insert_char(b' ' as i32);
        }

        Pos { x: self.cursor.x, y: self.cursor.y }
    }
}
