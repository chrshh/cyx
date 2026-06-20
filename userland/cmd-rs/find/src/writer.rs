use crate::{MATCH_FOUND, parse::Config, search::SearchResult};
use std::{fmt::Write as _, io::Write, sync::atomic::Ordering};

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
        writeln!(
            self.buf,
            "{}",
            self.highlight_match(
                &mut r.filepath.clone(),
                c.pattern.clone().unwrap().as_mut_str()
            ),
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
