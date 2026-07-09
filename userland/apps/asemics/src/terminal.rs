use std::ffi::CStr;
use std::sync::Mutex;

use crate::editor::clear_screen;

/* termios snapshot from before raw mode, restored on every exit path
 * (stands in for the C globals E.orig_term + the atexit(disableRawMode) hook) */
static ORIG_TERM: Mutex<Option<libc::termios>> = Mutex::new(None);

/* perror(3): "<s>: <strerror(errno)>" on stderr */
fn perror(s: &str) {
    let errno = std::io::Error::last_os_error().raw_os_error().unwrap_or(0);
    let msg = unsafe { CStr::from_ptr(libc::strerror(errno)) };
    eprintln!("{}: {}", s, msg.to_string_lossy());
}

pub fn strerror_last() -> String {
    let errno = std::io::Error::last_os_error().raw_os_error().unwrap_or(0);
    let msg = unsafe { CStr::from_ptr(libc::strerror(errno)) };
    msg.to_string_lossy().into_owned()
}

pub fn die(s: &str) -> ! {
    clear_screen();
    perror(s);
    exit_restore(1)
}

/* every exit funnels through here so the terminal is always restored,
 * mirroring the C atexit handler */
pub fn exit_restore(code: i32) -> ! {
    if let Ok(guard) = ORIG_TERM.lock() {
        if let Some(orig) = *guard {
            unsafe { libc::tcsetattr(libc::STDIN_FILENO, libc::TCSAFLUSH, &orig) };
        }
    }
    std::process::exit(code)
}

pub fn disable_raw_mode() {
    let orig = *ORIG_TERM.lock().unwrap();
    if let Some(orig) = orig {
        if unsafe { libc::tcsetattr(libc::STDIN_FILENO, libc::TCSAFLUSH, &orig) } == -1 {
            die("tcsetattr");
        }
    }
}

pub fn enable_raw_mode() {
    let mut orig: libc::termios = unsafe { std::mem::zeroed() };
    if unsafe { libc::tcgetattr(libc::STDIN_FILENO, &mut orig) } == -1 {
        die("tcgetattr");
    }
    *ORIG_TERM.lock().unwrap() = Some(orig);

    let mut raw = orig;
    raw.c_iflag &= !(libc::BRKINT | libc::ICRNL | libc::INPCK | libc::ISTRIP | libc::IXON);
    raw.c_oflag &= !libc::OPOST;
    raw.c_cflag |= libc::CS8;
    raw.c_lflag &= !(libc::ECHO | libc::ICANON | libc::IEXTEN | libc::ISIG);
    raw.c_cc[libc::VMIN] = 0;
    raw.c_cc[libc::VTIME] = 1;
    if unsafe { libc::tcsetattr(libc::STDIN_FILENO, libc::TCSAFLUSH, &raw) } == -1 {
        die("tcsetattr");
    }
}

/* the whole frame goes out in a single write(2), same as the C WriteBuf */
pub fn write_stdout(buf: &[u8]) -> isize {
    unsafe { libc::write(libc::STDOUT_FILENO, buf.as_ptr().cast(), buf.len()) }
}

pub fn get_window_size() -> Option<(i32, i32)> {
    let mut ws: libc::winsize = unsafe { std::mem::zeroed() };

    if unsafe { libc::ioctl(libc::STDOUT_FILENO, libc::TIOCGWINSZ, &mut ws) } == -1 || ws.ws_col == 0 {
        if write_stdout(b"\x1b[999C\x1b[999B") != 12 {
            return None;
        }
        get_cursor_position()
    } else {
        Some((ws.ws_row as i32, ws.ws_col as i32))
    }
}

pub fn get_cursor_position() -> Option<(i32, i32)> {
    let mut buf = [0u8; 32];
    let mut i = 0usize;

    if write_stdout(b"\x1b[6n") != 4 {
        return None;
    }

    while i < buf.len() - 1 {
        if unsafe { libc::read(libc::STDIN_FILENO, buf[i..].as_mut_ptr().cast(), 1) } != 1 {
            break;
        }
        if buf[i] == b'R' {
            break;
        }
        i += 1;
    }

    if buf[0] != 0x1b || buf[1] != b'[' {
        return None;
    }
    let s = std::str::from_utf8(&buf[2..i]).ok()?;
    let (rows, cols) = s.split_once(';')?;
    Some((rows.parse().ok()?, cols.parse().ok()?))
}
