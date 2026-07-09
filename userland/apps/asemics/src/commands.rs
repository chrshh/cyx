use crate::consts::{FORCE, QUIT, SAVE, SCREEN_CLEAR_LINE};
use crate::editor::{Editor, EditorMode};

pub fn parse_commands(cmd: &[u8]) -> i32 {
    let mut cmds = 0;
    if cmd.len() == 1 {
        return 0;
    }

    for &b in cmd.get(1..).unwrap_or(&[]) {
        match b {
            b'w' => cmds |= SAVE,
            b'q' => cmds |= QUIT,
            b'!' => cmds |= FORCE,
            _ => return -1,
        }
    }
    cmds
}

impl Editor {
    pub fn exec_commands(&mut self) {
        let cmds = parse_commands(&self.ui.cmdline);

        /* note: an unknown command parses to -1, which has every bit set, so it
         * behaves as save + force-quit — same as the C version */

        if cmds & SAVE != 0 {
            self.save();
        }
        if cmds & QUIT != 0 {
            self.quit(cmds & FORCE != 0);
        }

        self.mode = EditorMode::Normal;
    }

    pub fn command_insert_char(&mut self, c: i32) {
        let n = self.ui.cmdline.len();
        if n >= 80 {
            return;
        }
        self.ui.cmdline.push(c as u8);
    }

    pub fn command_del_char(&mut self) {
        let n = self.ui.cmdline.len();
        if n == 1 {
            return;
        }
        self.ui.cmdline.pop();
    }

    pub fn draw_cmdline(&self, wb: &mut Vec<u8>) {
        wb.extend_from_slice(SCREEN_CLEAR_LINE);
        let mut cmdlen = self.ui.cmdline.len() as i32;
        if cmdlen > self.viewport.width {
            cmdlen = self.viewport.width;
        }
        wb.extend_from_slice(&self.ui.cmdline[..cmdlen as usize]);
    }
}
