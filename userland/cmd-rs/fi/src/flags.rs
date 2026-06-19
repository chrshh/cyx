use crate::error::CGrepError;

pub const IGNORE_CASE: u32 = 1 << 0;
pub const RECURSIVE: u32 = 1 << 1;
pub const WHOLE_WORD: u32 = 1 << 2;
pub const LN_NUMS: u32 = 1 << 3;
pub const COUNT: u32 = 1 << 4;
pub const FILENAME_ONLY: u32 = 1 << 5;
pub const TIME: u32 = 1 << 6;

pub enum FlagConfig {
    Flags,
    NoFlags,
}

/* positioning for needle & haystack args */
impl FlagConfig {
    pub fn query_idx(&self) -> usize {
        match self {
            FlagConfig::NoFlags => 0,
            FlagConfig::Flags => 1,
        }
    }

    pub fn path_idx(&self) -> usize {
        match self {
            FlagConfig::NoFlags => 1,
            FlagConfig::Flags => 2,
        }
    }
}

pub fn parse_flags(flags: &str) -> Result<u32, CGrepError> {
    if !flags.starts_with('-') {
        return Ok(0);
    };

    let mut f = 0u32;
    for c in flags.chars() {
        match c {
            '-' => {}
            'i' => f |= IGNORE_CASE,
            'r' => f |= RECURSIVE,
            'w' => f |= WHOLE_WORD,
            'n' => f |= LN_NUMS,
            'c' => f |= COUNT,
            'l' => f |= FILENAME_ONLY,
            't' => f |= TIME,
            other => return Err(CGrepError::UnknownFlag(String::from(other))),
        }
    }

    Ok(f)
}
