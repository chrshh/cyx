fn main() {
    pub const SCREEN_CLEAR: &[u8] = b"\x1b[2J";
    pub const CURSOR_TL: &[u8] = b"\x1b[H";

    write_stdout(SCREEN_CLEAR);
    write_stdout(CURSOR_TL);
}

pub fn write_stdout(buf: &[u8]) -> isize {
    unsafe { libc::write(libc::STDOUT_FILENO, buf.as_ptr().cast(), buf.len()) }
}
