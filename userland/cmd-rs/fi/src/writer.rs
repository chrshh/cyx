use crate::{
    flags::{LN_NUMS, RECURSIVE},
    parse::Config,
    search::SearchResult,
};
use std::{fmt::Write as _, io::Write};

#[derive(Debug, Default)]
pub struct Output {
    buf: String,
}

impl Output {
    const GREEN: &'static str = "\x1b[32m";
    const PURPLE: &'static str = "\x1b[35m";
    const RED: &'static str = "\x1b[31m";
    const RESET: &'static str = "\x1b[0m";

    pub fn push_match(&mut self, r: SearchResult, c: &Config) {
        if c.flags & RECURSIVE != 0 {
            write!(
                self.buf,
                "{}{}{}:",
                Output::PURPLE,
                r.filepath,
                Output::RESET
            )
            .unwrap();
        }

        if c.flags & LN_NUMS != 0 {
            write!(
                self.buf,
                "{}{}{}:\t",
                Output::GREEN,
                r.line_num,
                Output::RESET
            )
            .unwrap();
        }

        writeln!(self.buf, "{}{}{}", Output::RED, r.complement, Output::RESET).unwrap();
    }

    pub fn flush_to<W: Write>(&mut self, out: &mut W) {
        if !self.buf.is_empty() {
            out.write_all(self.buf.as_bytes()).unwrap();
            self.buf.clear();
        }
    }
}
