use std::fs;
use std::io::Write;
use std::os::unix::fs::OpenOptionsExt;
use std::time::SystemTime;

use crate::consts::*;
use crate::dbg::init_dbg;
use crate::history::History;
use crate::input::{read_key, BACKSPACE, ARROW_DOWN, ARROW_LEFT, ARROW_RIGHT, ARROW_UP};
use crate::rows::Row;
use crate::syntax::{syntax_to_color, EditorSyntax, HL_NORMAL};
use crate::terminal::{die, exit_restore, get_window_size, strerror_last, write_stdout};

/* global state */
#[derive(Clone, Copy, PartialEq)]
pub enum EditorMode {
    Normal,
    Insert,
    Command,
    Visual,
}

#[derive(Clone, Copy)]
pub struct Cursor {
    pub x: i32,  /* cursor column in the buffer (logical char index, ignores tabs) */
    pub y: i32,  /* cursor row in the buffer (line index) */
    pub rx: i32, /* cursor column on screen (`x` with tabs expanded) */
}

pub struct Viewport {
    pub height: i32,  /* number of visible rows on screen */
    pub width: i32,   /* number of visible columns on screen */
    pub row_off: i32, /* topmost visible buffer row (vertical scroll offset) */
    pub col_off: i32, /* leftmost visible column (horizontal scroll offset) */
}

pub struct Buffer {
    pub rows: Vec<Row>, /* the document contents */
    /* kept separately from rows.len(): the C code bumps num_rows only after the
     * syntax update runs during a row insert, and comment propagation reads it */
    pub num_rows: i32,
    pub dirty: bool,              /* true if there are unsaved changes */
    pub filename: Option<String>, /* path to the open file (None when unnamed) */
}

pub struct StatusBar {
    pub msg: Vec<u8>,            /* transient status message shown at the bottom */
    pub msg_time: SystemTime,    /* when `msg` was set, used to time it out */
    pub cmdline: Vec<u8>,        /* text being typed in command mode (`:w`, `:q`, ...) */
}

/* position struct for motions */
#[derive(Clone, Copy)]
pub struct Pos {
    pub x: i32,
    pub y: i32,
}

pub struct Editor {
    pub mode: EditorMode,                      /* current modal state */
    pub cursor: Cursor,                        /* where the cursor is */
    pub viewport: Viewport,                    /* what slice of the buffer is visible */
    pub buffer: Buffer,                        /* the document being edited */
    pub ui: StatusBar,                         /* bottom-bar state: status message + command line */
    pub syntax: Option<&'static EditorSyntax>, /* active syntax-highlight rules (None = none) */
    pub history: History,
    pub dbg: fs::File,
}

pub fn is_cntrl_byte(c: u8) -> bool {
    c < 32 || c == 127
}

/* iscntrl(3) as the C code uses it: readKey returns a sign-extended char, so
 * bytes >= 128 arrive negative and are not control chars */
pub fn is_cntrl_int(c: i32) -> bool {
    (0..32).contains(&c) || c == 127
}

impl Editor {
    pub fn new() -> Editor {
        let dbg = init_dbg();

        let Some((mut height, mut width)) = get_window_size() else {
            die("getWindowSize")
        };
        height -= STATUS_BAR_RESERVE; // status bar space for bottom of screen
        width -= LINE_NUM_RESERVE;

        Editor {
            mode: EditorMode::Normal,
            cursor: Cursor { x: 0, y: 0, rx: 0 },
            viewport: Viewport {
                height,
                width,
                row_off: 0,
                col_off: 0,
            },
            buffer: Buffer {
                rows: Vec::new(),
                num_rows: 0,
                dirty: false,
                filename: None,
            },
            ui: StatusBar {
                msg: Vec::new(),
                msg_time: SystemTime::UNIX_EPOCH,
                cmdline: Vec::new(),
            },
            syntax: None,
            history: History::new(),
            dbg,
        }
    }

    /* draw fn for entire editor */
    pub fn draw_rows(&self, wb: &mut Vec<u8>) {
        for y in 0..self.viewport.height {
            let filerow = y + self.viewport.row_off;

            if filerow >= self.buffer.num_rows {
                if self.buffer.num_rows == 0 && y == self.viewport.height / 3 {
                    /* welcome screen rendered when no file is selected */
                    self.welcome_screen(wb);
                } else {
                    wb.push(b'~');
                }
            } else {
                self.draw_line_nums(wb, filerow);

                let row = &self.buffer.rows[filerow as usize];
                let mut len = row.rsize() - self.viewport.col_off;
                if len < 0 {
                    len = 0;
                }
                if len > self.viewport.width {
                    len = self.viewport.width;
                }

                let start = self.viewport.col_off as usize;
                let mut curr_color: Option<&'static str> = None;

                for j in 0..len as usize {
                    let c = row.render[start + j];
                    let hl = row.hl[start + j];

                    if is_cntrl_byte(c) {
                        let sym = if c <= 26 { b'@' + c } else { b'?' };
                        wb.extend_from_slice(b"\x1b[7m");
                        wb.push(sym);
                        wb.extend_from_slice(b"\x1b[m");

                        if let Some(color) = curr_color {
                            wb.extend_from_slice(color.as_bytes());
                        }
                    } else if hl == HL_NORMAL {
                        if curr_color.is_some() {
                            wb.extend_from_slice(DEF_COLOR.as_bytes());
                            curr_color = None;
                        }
                        wb.push(c);
                    } else {
                        let color = syntax_to_color(hl);
                        if Some(color) != curr_color {
                            curr_color = Some(color);
                            wb.extend_from_slice(color.as_bytes());
                        }
                        wb.push(c);
                    }
                }
                wb.extend_from_slice(DEF_COLOR.as_bytes());
            }

            wb.extend_from_slice(SCREEN_CLEAR_LINE);
            wb.extend_from_slice(b"\r\n");
        }
    }

    pub fn draw_line_nums(&self, wb: &mut Vec<u8>, filerow: i32) {
        /* highlight current line num for cursor */
        let gutter = if self.cursor.y == filerow {
            format!("{:5}  ", filerow + 1)
        } else {
            format!("{}{:5}  {}", DARK_GRAY, filerow + 1, RESET_FG)
        };
        wb.extend_from_slice(gutter.as_bytes());
    }

    pub fn welcome_screen(&self, wb: &mut Vec<u8>) {
        let welcome_title = "asemics -- 0.1";
        let welcome_desc1 = "Asemic - mark-making that resembles text";
        let welcome_desc2 = " or handwriting but carries no specific literal meaning.";

        let mut titlelen = welcome_title.len() as i32;
        let mut desclen1 = welcome_desc1.len() as i32;
        let mut desclen2 = welcome_desc2.len() as i32;

        if titlelen > self.viewport.width {
            titlelen = self.viewport.width;
        }
        if desclen1 > self.viewport.width {
            desclen1 = self.viewport.width;
        }
        if desclen2 > self.viewport.width {
            desclen2 = self.viewport.width;
        }

        let mut title_padding = (self.viewport.width - titlelen) / 2;
        let desc_padding1 = (self.viewport.width - desclen1) / 2;
        let desc_padding2 = (self.viewport.width - desclen2) / 2;

        wb.extend_from_slice(CURSOR_HIDE);

        if title_padding != 0 {
            wb.push(b'~');
            title_padding -= 1;
        }

        /* title */
        for _ in 0..title_padding {
            wb.push(b' ');
        }
        wb.extend_from_slice(&welcome_title.as_bytes()[..titlelen as usize]);
        wb.extend_from_slice(b"\r\n\n");

        /* desc 1 */
        for _ in 0..desc_padding1 {
            wb.push(b' ');
        }
        wb.extend_from_slice(&welcome_desc1.as_bytes()[..desclen1 as usize]);
        wb.extend_from_slice(b"\r\n");

        /* desc 2 */
        for _ in 0..desc_padding2 {
            wb.push(b' ');
        }
        wb.extend_from_slice(&welcome_desc2.as_bytes()[..desclen2 as usize]);
    }

    pub fn update_cursor_shape(&self, wb: &mut Vec<u8>) {
        match self.mode {
            EditorMode::Normal => wb.extend_from_slice(CURSOR_BLOCK),
            EditorMode::Command => wb.extend_from_slice(CURSOR_HIDE),
            EditorMode::Insert => wb.extend_from_slice(CURSOR_BAR),
            EditorMode::Visual => wb.extend_from_slice(CURSOR_BLOCK),
        }
    }

    pub fn refresh_screen(&mut self) {
        self.scroll();
        let mut wb: Vec<u8> = Vec::new();

        wb.extend_from_slice(CURSOR_HIDE);
        wb.extend_from_slice(SCREEN_CLEAR);
        wb.extend_from_slice(CURSOR_TL);
        self.draw_rows(&mut wb);

        /* status bar buffer */
        wb.extend_from_slice(format!("\x1b[{};1H", self.viewport.height + 1).as_bytes());
        self.draw_status_bar(&mut wb);

        /* message bar buffer */
        wb.extend_from_slice(format!("\x1b[{};1H", self.viewport.height + 2).as_bytes());
        if !self.ui.cmdline.is_empty() {
            self.draw_cmdline(&mut wb);
        } else {
            self.draw_msg_bar(&mut wb);
        }

        /* line number gutter buffer */
        wb.extend_from_slice(
            format!(
                "\x1b[{};{}H",
                (self.cursor.y - self.viewport.row_off) + 1,
                (self.cursor.rx - self.viewport.col_off) + LINE_NUM_RESERVE // consumes 8 visual cols
            )
            .as_bytes(),
        );

        /* Enabled cursor and render cursor based on EDITOR MODE && if a file is open */
        if self.buffer.filename.is_some() {
            wb.extend_from_slice(CURSOR_SHOW);
            self.update_cursor_shape(&mut wb);
        } else {
            wb.extend_from_slice(CURSOR_HIDE);
        }

        write_stdout(&wb);
    }

    pub fn move_cursor(&mut self, key: i32) {
        let row_size = |e: &Editor| -> Option<i32> {
            if e.cursor.y >= e.buffer.num_rows {
                None
            } else {
                Some(e.buffer.rows[e.cursor.y as usize].size())
            }
        };

        let row = row_size(self);
        if key == b'h' as i32 || key == ARROW_LEFT {
            if self.cursor.x != 0 {
                self.cursor.x -= 1;
            }
        } else if key == b'j' as i32 || key == ARROW_DOWN {
            if self.cursor.y < self.buffer.num_rows - 1 {
                self.cursor.y += 1;
            }
        } else if key == b'k' as i32 || key == ARROW_UP {
            if self.cursor.y != 0 {
                self.cursor.y -= 1;
            }
        } else if key == b'l' as i32 || key == ARROW_RIGHT {
            if let Some(size) = row {
                if self.cursor.x < size - 1 {
                    self.cursor.x += 1;
                }
            }
        }

        let rowlen = row_size(self).unwrap_or(0);
        if self.cursor.x > rowlen {
            self.cursor.x = rowlen;
        }
    }

    pub fn create_file(filename: &str) -> i32 {
        match fs::OpenOptions::new()
            .create(true)
            .write(true)
            .mode(0o644)
            .open(filename)
        {
            Ok(_) => 0,
            Err(_) => {
                eprintln!("{}: {}", filename, strerror_last());
                1
            }
        }
    }

    pub fn open(&mut self, filename: &str) {
        self.buffer.filename = Some(filename.to_string());

        self.set_syntax_highlight();

        let data = match fs::read(filename) {
            Ok(d) => d,
            Err(_) => {
                if Editor::create_file(filename) != 0 {
                    die("open & create");
                }
                match fs::read(filename) {
                    Ok(d) => d,
                    Err(_) => die("open & create"),
                }
            }
        };

        for chunk in data.split_inclusive(|&b| b == b'\n') {
            let mut line = chunk;
            while let Some((&last, rest)) = line.split_last() {
                if last == b'\n' || last == b'\r' {
                    line = rest;
                } else {
                    break;
                }
            }
            self.insert_row(self.buffer.num_rows, line);
        }
        self.buffer.dirty = false;
    }

    /* y = 0: top of file  */
    pub fn scroll(&mut self) {
        /* obtain cursor column */
        self.cursor.rx = 0;
        if self.cursor.y < self.buffer.num_rows {
            self.cursor.rx = self.buffer.rows[self.cursor.y as usize].x_to_rx(self.cursor.x);
        }

        /* scrolloff for approaching top of file */
        if self.cursor.y < self.viewport.row_off + SCROLL_OFF {
            self.viewport.row_off = self.cursor.y - SCROLL_OFF;
            if self.viewport.row_off < 0 {
                self.viewport.row_off = 0;
            }
        }

        /* scrolloff for approaching bottom of file */
        if self.cursor.y >= self.viewport.row_off + self.viewport.height - SCROLL_OFF {
            self.viewport.row_off = self.cursor.y - self.viewport.height + 1 + SCROLL_OFF;
        }

        if self.cursor.rx < self.viewport.col_off {
            self.viewport.col_off = self.cursor.rx;
        }
        if self.cursor.rx >= self.viewport.col_off + self.viewport.width {
            self.viewport.col_off = self.cursor.rx - self.viewport.width + 1;
        }
    }

    pub fn rows_to_string(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        for row in &self.buffer.rows[..self.buffer.num_rows as usize] {
            buf.extend_from_slice(&row.chars);
            buf.push(b'\n');
        }
        buf
    }

    pub fn save(&mut self) {
        let Some(filename) = self.buffer.filename.clone() else {
            return;
        };

        let buf = self.rows_to_string();
        let len = buf.len();

        if let Ok(mut file) = fs::OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .mode(0o644)
            .open(&filename)
        {
            if file.set_len(len as u64).is_ok() {
                if file.write_all(&buf).is_ok() {
                    self.buffer.dirty = false;
                    self.ui.cmdline.clear();
                    self.set_status_msg(format!("{}: bytes written to disk", len));
                    self.set_syntax_highlight();
                    return;
                }
                self.set_syntax_highlight();
            }
        }
        self.set_status_msg(format!("Failed to save. I/O error: {}", strerror_last()));
    }

    pub fn quit(&mut self, force: bool) {
        if force {
            clear_screen();
            exit_restore(0);
        } else if self.buffer.dirty {
            let msg = format!(
                "{} has unsaved changes. '!q' to quit without saving",
                self.buffer.filename.as_deref().unwrap_or("(null)")
            );
            let mut msg = msg.into_bytes();
            msg.truncate(79); // C builds this in a char buf[80]
            self.ui.cmdline.clear();
            for &b in &msg {
                self.command_insert_char(b as i32);
            }
        } else {
            clear_screen();
            exit_restore(0);
        }
    }

    pub fn prompt(
        &mut self,
        prompt_fmt: &str,
        mut callback: impl FnMut(&mut Editor, &[u8], i32),
    ) -> Option<Vec<u8>> {
        let mut buf: Vec<u8> = Vec::new();

        loop {
            let shown = prompt_fmt.replace("%s", &String::from_utf8_lossy(&buf));
            self.set_status_msg(shown);
            self.refresh_screen();

            let c = read_key();
            if c == BACKSPACE {
                if !buf.is_empty() {
                    buf.pop();
                }
            } else if c == ESC {
                self.set_status_msg(String::new());
                callback(self, &buf, c);
                return None;
            } else if c == ENTER {
                if !buf.is_empty() {
                    self.set_status_msg(String::new());
                    callback(self, &buf, c);
                    return Some(buf);
                }
            } else if !is_cntrl_int(c) && c < 128 {
                buf.push(c as u8);
            }

            callback(self, &buf, c);
        }
    }
}

pub fn clear_screen() {
    write_stdout(SCREEN_CLEAR);
    write_stdout(CURSOR_TL);
}
