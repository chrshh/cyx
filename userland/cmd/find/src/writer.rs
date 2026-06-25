use crate::{MATCH_FOUND, parse::Config, search::SearchResult};
use std::{fmt::Write as _, io::Write, sync::atomic::Ordering};

#[derive(Debug, Default)]
pub struct Output {
    buf: String,
}

impl Output {
    const RED: &'static str = "\x1b[31m";
    const RESET: &'static str = "\x1b[0m";

    pub fn push_match(&mut self, r: SearchResult, c: &Config) {
        let pattern = c.pattern.as_deref().unwrap_or("");
        writeln!(
            self.buf,
            "{}",
            self.highlight_match(&r.filepath, pattern),
        )
        .unwrap();
    }

    pub fn highlight_match(&self, line: &str, pattern: &str) -> String {
        if pattern.is_empty() {
            return line.to_string();
        }
        line.replace(
            pattern,
            &format!("{}{}{}", Output::RED, pattern, Output::RESET),
        )
    }

    pub fn post_search_output(&mut self) {
        if !MATCH_FOUND.load(Ordering::Relaxed) {
            writeln!(self.buf, "cfind: no matches found").unwrap();
        }
    }

    pub fn flush_to<W: Write>(&mut self, out: &mut W) {
        if !self.buf.is_empty() {
            out.write_all(self.buf.as_bytes()).unwrap();
            self.buf.clear();
        }
    }
}
