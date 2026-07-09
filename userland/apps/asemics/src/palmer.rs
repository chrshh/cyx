use std::io::Read;
use std::process::{Command, Stdio};

use crate::consts::PALMER_MAX_PATH;
use crate::editor::Editor;
use crate::terminal::{disable_raw_mode, enable_raw_mode};

impl Editor {
    /* on success opens the chosen file in editor and returns 0 */
    /* on failure returns -1 */
    pub fn invoke_palmer(&mut self, start_dir: Option<&str>) -> i32 {
        /* palmer owns the terminal now */
        disable_raw_mode();

        // XXX Swap this out for prod path later
        let palmer_bin = "/home/chris/repositories/cjyx/userland/target/debug/palmer";

        /*
         * palmer draws its UI on /dev/tty and prints only the chosen path to
         * stdout, which we capture through a pipe; stdin/stderr stay on the tty
         */
        let mut cmd = Command::new(palmer_bin);
        cmd.arg("--pick-file");
        if let Some(dir) = start_dir {
            cmd.args(["--cwd", dir]);
        }
        cmd.stdout(Stdio::piped());

        let mut child = match cmd.spawn() {
            Ok(child) => child,
            Err(_) => {
                enable_raw_mode();
                return -1;
            }
        };

        /* drain palmers stdout into a buffer until palmer pipe closes */
        let mut path: Vec<u8> = Vec::new();
        let mut stdout = child.stdout.take().unwrap();
        let _ = stdout
            .by_ref()
            .take((PALMER_MAX_PATH - 1) as u64)
            .read_to_end(&mut path);
        drop(stdout);

        let status = child.wait();

        /* return terminal to asemics */
        enable_raw_mode();

        let exited_ok = matches!(&status, Ok(s) if s.success());
        if !exited_ok || path.is_empty() {
            return -1;
        }

        /* strip newline palmer prints */
        if path.last() == Some(&b'\n') {
            path.pop();
        }

        let path = String::from_utf8_lossy(&path).into_owned();
        self.open(&path);
        self.set_status_msg(format!("opened {}", path));
        0
    }
}
