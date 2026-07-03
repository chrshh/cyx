use std::time::SystemTime;

use crate::consts::{LINE_NUM_RESERVE, SCREEN_CLEAR_LINE};
use crate::editor::{Editor, EditorMode};

impl Editor {
    pub fn mode_str(&self) -> &'static str {
        match self.mode {
            EditorMode::Insert => "-- INSERT -- ",
            EditorMode::Command => "-- COMMAND -- ",
            EditorMode::Visual => "-- VISUAL -- ",
            EditorMode::Normal => "-- NORMAL -- ",
        }
    }

    pub fn draw_status_bar(&self, wb: &mut Vec<u8>) {
        wb.extend_from_slice(b"\x1b[7m");

        let mdstatus = format!("{:<10}", self.mode_str());

        let status = format!(
            "{:.20} - {} lines {}",
            self.buffer.filename.as_deref().unwrap_or("[No Name]"),
            self.buffer.num_rows,
            if self.buffer.dirty { "*" } else { "" }
        );

        let rstatus = format!(
            "{} | {}/{}",
            self.syntax.map_or("no ft", |s| s.filetype),
            self.cursor.y + 1,
            self.buffer.num_rows
        );

        let mut modelen = mdstatus.len() as i32;
        let mut statuslen = status.len() as i32;
        let mut rlen = rstatus.len() as i32;

        if modelen > self.viewport.width {
            modelen = self.viewport.width;
        }
        if modelen + statuslen > self.viewport.width {
            statuslen = self.viewport.width - modelen;
        }
        if modelen + statuslen + rlen > self.viewport.width {
            rlen = self.viewport.width - modelen - statuslen;
        }
        if statuslen < 0 {
            statuslen = 0;
        }
        if rlen < 0 {
            rlen = 0;
        }

        wb.extend_from_slice(&mdstatus.as_bytes()[..modelen as usize]);
        wb.extend_from_slice(&status.as_bytes()[..statuslen as usize]);

        let mut written = modelen + statuslen;
        while written < self.viewport.width + LINE_NUM_RESERVE - rlen {
            wb.push(b' ');
            written += 1;
        }
        wb.extend_from_slice(&rstatus.as_bytes()[..rlen as usize]);
        wb.extend_from_slice(b"\x1b[m");
    }

    pub fn set_status_msg(&mut self, msg: String) {
        let mut msg = msg.into_bytes();
        msg.truncate(79); // C formats into a char msg[80]
        self.ui.msg = msg;
        self.ui.msg_time = SystemTime::now();
    }

    pub fn draw_msg_bar(&self, wb: &mut Vec<u8>) {
        wb.extend_from_slice(SCREEN_CLEAR_LINE);
        let mut msglen = self.ui.msg.len() as i32;
        if msglen > self.viewport.width {
            msglen = self.viewport.width;
        }
        let recent = self.ui.msg_time.elapsed().is_ok_and(|d| d.as_secs() < 5);
        if msglen > 0 && recent {
            wb.extend_from_slice(&self.ui.msg[..msglen as usize]);
        }
    }
}
