/* keys */
pub const fn ctrl_key(k: u8) -> i32 {
    (k & 0x1f) as i32
}

pub const ENTER: i32 = b'\r' as i32;
pub const ESC: i32 = 0x1b;

pub const LEADER: i32 = 32;

/* cursor & screen */
pub const CURSOR_TL: &[u8] = b"\x1b[H";
pub const CURSOR_HIDE: &[u8] = b"\x1b[?25l";
pub const CURSOR_SHOW: &[u8] = b"\x1b[?25h";
pub const CURSOR_BLOCK: &[u8] = b"\x1b[2 q";
pub const CURSOR_BAR: &[u8] = b"\x1b[6 q";

pub const SCREEN_CLEAR: &[u8] = b"\x1b[2J";
pub const SCREEN_CLEAR_LINE: &[u8] = b"\x1b[K";

pub const TAB_STOP: i32 = 8;

pub const STATUS_BAR_RESERVE: i32 = 2;
pub const LINE_NUM_RESERVE: i32 = 8;

pub const SCROLL_OFF: i32 = 5;

pub const PALMER_MAX_PATH: usize = 4096;

/* commands */
pub const SAVE: i32 = 1 << 0;
pub const QUIT: i32 = 1 << 1;
pub const FORCE: i32 = 1 << 2;

/* colors */
pub const DARK_GRAY: &str = "\x1b[90m";
pub const BLUE: &str = "\x1b[94m";
pub const GREEN: &str = "\x1b[38;2;165;214;255m";
pub const KEYWORD1: &str = "\x1b[31m";
pub const KEYWORD2: &str = "\x1b[31m";
pub const DEF_COLOR: &str = "\x1b[39m";
pub const COMMENT: &str = "\x1b[90m";
pub const MATCH: &str = "\x1b[35m";
pub const OPERATOR: &str = BLUE;
pub const RESET_FG: &str = "\x1b[39m";
