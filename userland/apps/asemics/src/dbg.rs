use std::fs::File;
use std::io::Write;

use crate::consts::SCREEN_CLEAR;
use crate::editor::Editor;
use crate::terminal::die;

pub fn init_dbg() -> File {
    let Ok(mut dbg) = File::create("/tmp/asemics.log") else {
        die("fopen")
    };
    let _ = dbg.write_all(SCREEN_CLEAR);
    let _ = dbg.write_all(b"-- ENTRY --\n");
    dbg
}

impl Editor {
    pub fn add_dbg_log(&mut self, msg: &str) {
        let _ = write!(
            self.dbg,
            " ({}, {}) MODE={}\n{}",
            self.cursor.rx, self.cursor.y, self.mode as i32, msg
        );
    }
}
