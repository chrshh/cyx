use std::{
    env,
    fs::File,
    io::{self, ErrorKind, Read},
    os::{
        fd::{AsRawFd, OwnedFd},
        unix::process::CommandExt,
    },
    process::{Child, Command},
};

use rustix::termios::tcsetwinsize;
use rustix::{
    fs::{OFlags, fcntl_setfl},
    process::setsid,
    termios::Winsize,
};
use rustix_openpty::openpty;

pub struct Pty {
    pub child: Child,
    pub file: File,
    pub cols: u16,
    pub rows: u16,
}

// Tha TTY, PTY
pub fn new(cols: u16, rows: u16) -> io::Result<Pty> {
    let pty = openpty(None, None)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("openpty: {e}")))?;

    let master: OwnedFd = pty.controller;
    let slave: OwnedFd = pty.user;

    let master_raw = master.as_raw_fd();

    let shell = env::var("SHELL").unwrap_or_else(|_| "/bin/sh".to_string());
    let mut cmd = Command::new(shell);
    cmd.env("TERM", "xterm-256color");
    cmd.env("COLORTERM", "truecolor");

    let slave_for_child = slave;
    unsafe {
        cmd.pre_exec(move || {
            setsid().map_err(io::Error::from)?;

            let r = libc::ioctl(slave_for_child.as_raw_fd(), libc::TIOCSCTTY as _, 0);
            if r != 0 {
                return Err(io::Error::last_os_error());
            }

            let sfd = slave_for_child.as_raw_fd();
            if libc::dup2(sfd, 0) < 0 || libc::dup2(sfd, 1) < 0 || libc::dup2(sfd, 2) < 0 {
                return Err(io::Error::last_os_error());
            }

            if sfd > 2 {
                libc::close(sfd);
            }
            libc::close(master_raw);
            Ok(())
        });
    }

    let child = cmd.spawn()?;

    let file = File::from(master);
    fcntl_setfl(&file, OFlags::NONBLOCK)?;

    let ws = Winsize {
        ws_row: rows,
        ws_col: cols,
        ws_xpixel: 0,
        ws_ypixel: 0,
    };

    tcsetwinsize(&file, ws)?;

    Ok(Pty {
        child,
        file,
        cols,
        rows,
    })
}

impl Pty {
    // Test print to console
    pub fn read_from_file(&mut self) {
        eprintln!("read_from_file start");
        let mut buf = [0u8; 4096];
        loop {
            match (&self.file).read(&mut buf) {
                Ok(0) => {
                    eprintln!("EOF — shell exited");
                    return;
                }
                Ok(n) => {
                    eprintln!("read {} bytes:", n);
                    for b in &buf[..n] {
                        if b.is_ascii_graphic() || *b == b' ' {
                            eprint!("{}", *b as char);
                        } else {
                            eprint!("\\x{:02x}", b);
                        }
                    }
                    eprintln!();
                }
                Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                    std::thread::sleep(std::time::Duration::from_millis(10));
                }
                Err(e) => {
                    eprintln!("read error: {e}");
                    return;
                }
            }
        }
    }
}
