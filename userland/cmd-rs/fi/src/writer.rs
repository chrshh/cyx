use crate::{
    MATCH_FOUND,
    flags::{LN_NUMS, RECURSIVE, TIME},
    parse::Config,
    search::SearchResult,
};
use std::{fmt::Write as _, io::Write, sync::atomic::Ordering, time::Duration};

#[derive(Debug, Default)]
pub struct Output {
    buf: String,
}

impl Output {
    const GREEN: &'static str = "\x1b[32m";
    const PURPLE: &'static str = "\x1b[35m";
    const RED: &'static str = "\x1b[31m";
    const YELLOW: &'static str = "\x1b[33m";
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

        writeln!(
            self.buf,
            "{}",
            self.highlight_match(&mut r.complement.clone(), &c.pattern),
        )
        .unwrap();
    }

    pub fn highlight_match(&self, line: &mut str, pattern: &str) -> String {
        let line = &line.replace(
            pattern,
            &format!("{}{}{}", Output::RED, pattern, Output::RESET),
        );

        String::from(line)
    }

    pub fn post_search_output(&mut self, c: &Config, end: Duration) {
        if c.flags & TIME != 0 {
            writeln!(
                self.buf,
                "cgrep: Query time: {}{:?}{}",
                Output::YELLOW,
                end,
                Output::RESET
            )
            .unwrap();
        }

        if !MATCH_FOUND.load(Ordering::Relaxed) {
            writeln!(self.buf, "cgrep: no matches found").unwrap();
        }
    }

    pub fn flush_to<W: Write>(&mut self, out: &mut W) {
        if !self.buf.is_empty() {
            out.write_all(self.buf.as_bytes()).unwrap();
            self.buf.clear();
        }
    }
}
