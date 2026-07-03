use crate::consts::*;
use crate::editor::{clear_screen, Editor, EditorMode};
use crate::terminal::{die, exit_restore};

pub const BACKSPACE: i32 = 127;
pub const ARROW_LEFT: i32 = 1000;
pub const ARROW_RIGHT: i32 = 1001;
pub const ARROW_UP: i32 = 1002;
pub const ARROW_DOWN: i32 = 1003;

pub fn read_key() -> i32 {
    let mut c: u8 = 0;

    loop {
        let nread = unsafe { libc::read(libc::STDIN_FILENO, (&mut c as *mut u8).cast(), 1) };
        if nread == 1 {
            break;
        }
        if nread == -1
            && std::io::Error::last_os_error().raw_os_error() != Some(libc::EAGAIN)
        {
            die("read");
        }
    }

    /* ESC sequences */
    if c == 0x1b {
        let mut seq = [0u8; 2];

        if unsafe { libc::read(libc::STDIN_FILENO, seq.as_mut_ptr().cast(), 1) } != 1 {
            return ESC;
        }
        if unsafe { libc::read(libc::STDIN_FILENO, seq[1..].as_mut_ptr().cast(), 1) } != 1 {
            return ESC;
        }

        if seq[0] == b'[' {
            match seq[1] {
                b'A' => return ARROW_UP,
                b'B' => return ARROW_DOWN,
                b'C' => return ARROW_RIGHT,
                b'D' => return ARROW_LEFT,
                _ => {}
            }
        }

        ESC
    } else {
        /* C returns a plain (signed) char here, so bytes >= 128 come back
         * negative; keep that so comparisons and truncations match */
        c as i8 as i32
    }
}

impl Editor {
    pub fn process_key(&mut self) {
        let c = read_key();

        match self.mode {
            EditorMode::Insert => self.handle_insert_mode_key(c),
            EditorMode::Normal => self.handle_normal_mode_key(c),
            EditorMode::Command => self.handle_command_mode_key(c),
            EditorMode::Visual => self.handle_visual_mode_key(c),
        }
    }

    pub fn handle_normal_mode_key(&mut self, c: i32) {
        /* QUIT */
        if c == ctrl_key(b'c') {
            clear_screen();
            exit_restore(0);
        }
        /* special actions */
        else if c == ctrl_key(b's') {
            self.save();
        } else if c == b'/' as i32 {
            self.find();
        } else if c == LEADER {
            self.handle_leader_key();
        } else if c == b'u' as i32 {
            self.history_undo();
        } else if c == ctrl_key(b'r') {
            self.history_redo();
        }
        /* ***** motion keys only ***** */
        else if c == b'o' as i32 {
            self.history_checkpoint();
            let target = self.action_insert_line_below_cursor();
            self.cursor.x = target.x;
            self.cursor.y = target.y;
            self.mode = EditorMode::Insert;
        } else if c == b'O' as i32 {
            self.history_checkpoint();
            let target = self.action_insert_line_above_cursor();
            self.cursor.x = target.x;
            self.cursor.y = target.y;
            self.mode = EditorMode::Insert;
        }
        /* COMPLEX MOTIONS */
        else if c == b'G' as i32 {
            /* the C loop is `while (scrl_down--)`, which underflows on an empty
             * buffer; skip instead of spinning */
            for _ in 0..self.buffer.num_rows - 1 {
                self.move_cursor(b'j' as i32);
            }
        } else if c == b'g' as i32 {
            let d = read_key();
            if d == b'g' as i32 {
                self.cursor.y = 0;
            }
            /* C fallthrough: 'g' has no break and falls into 'w' */
            let target = self.motion_word_forward();
            self.cursor.x = target.x;
            self.cursor.y = target.y;
        } else if c == b'w' as i32 {
            let target = self.motion_word_forward();
            self.cursor.x = target.x;
            self.cursor.y = target.y;
        } else if c == b'W' as i32 {
            let target = self.motion_word_forward_big();
            self.cursor.x = target.x;
            self.cursor.y = target.y;
        } else if c == b'e' as i32 {
            let target = self.motion_word_end();
            self.cursor.x = target.x;
            self.cursor.y = target.y;
        } else if c == b'E' as i32 {
            let target = self.motion_word_end_big();
            self.cursor.x = target.x;
            self.cursor.y = target.y;
        } else if c == b'b' as i32 {
            let target = self.motion_word_backwards();
            self.cursor.x = target.x;
            self.cursor.y = target.y;
        } else if c == b'B' as i32 {
            let target = self.motion_word_backwards_big();
            self.cursor.x = target.x;
            self.cursor.y = target.y;
            /* C fallthrough: 'B' has no break and lands on moveCursor('B'),
             * which matches no direction and only re-clamps the cursor */
            self.move_cursor(c);
        }
        /* BASIC MOTIONS */
        else if c == b'h' as i32
            || c == b'j' as i32
            || c == b'k' as i32
            || c == b'l' as i32
            || c == ARROW_UP
            || c == ARROW_DOWN
            || c == ARROW_LEFT
            || c == ARROW_RIGHT
        {
            self.move_cursor(c);
        } else if c == b'$' as i32 {
            let target = self.motion_line_last_char();
            self.cursor.x = target.x;
            self.cursor.y = target.y;
        } else if c == b'^' as i32 {
            self.cursor.x = 0;
        }
        /* enter insert mode */
        else if c == b'i' as i32 {
            if self.buffer.filename.is_none() {
                return;
            }
            self.history_checkpoint();
            self.mode = EditorMode::Insert;
        } else if c == b'a' as i32 {
            self.history_checkpoint();
            self.mode = EditorMode::Insert;
            self.cursor.x += 1;
        } else if c == b':' as i32 {
            self.mode = EditorMode::Command;
            self.ui.cmdline = vec![b':'];
        } else if c == b'v' as i32 {
            self.mode = EditorMode::Visual;
        }
    }

    pub fn handle_command_mode_key(&mut self, c: i32) {
        /* QUIT */
        if c == ctrl_key(b'q') {
            clear_screen();
            exit_restore(0);
        } else if c == ENTER {
            self.exec_commands();
        } else if c == BACKSPACE {
            self.command_del_char();
        } else if c == ESC {
            self.mode = EditorMode::Normal;
            self.ui.cmdline.clear();
        } else {
            self.command_insert_char(c);
        }
    }

    pub fn handle_visual_mode_key(&mut self, c: i32) {
        if c == ESC {
            self.mode = EditorMode::Normal;
        }
    }

    pub fn handle_insert_mode_key(&mut self, c: i32) {
        /* enter key */
        if c == ENTER {
            self.insert_new_line();
        }
        /* QUIT */
        else if c == ctrl_key(b'q') {
            self.quit(false);
        } else if c == BACKSPACE {
            self.del_char();
        } else if c == ESC {
            self.mode = EditorMode::Normal;
            self.cursor.x -= 1;
        } else if c == ARROW_DOWN || c == ARROW_UP || c == ARROW_LEFT || c == ARROW_RIGHT {
            self.move_cursor(c);
        } else {
            self.insert_char(c);
        }
    }

    pub fn handle_leader_key(&mut self) {
        let c = read_key();

        /* <leader>f? */
        if c == b'f' as i32 {
            let f = read_key();
            if f == b'f' as i32 {
                self.invoke_palmer(None);
            } else if f == b'F' as i32 {
                let home = std::env::var("HOME").ok();
                self.invoke_palmer(home.as_deref());
            }
        }
    }
}
