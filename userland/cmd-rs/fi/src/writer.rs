use crate::{
    flags::{LN_NUMS, RECURSIVE},
    parse::Config,
    search::SearchResult,
};
use std::fmt::Write;

// \x1b[32m sets text to green
// \x1b[0m resets the color to default
#[derive(Debug, Default)]
pub struct Output;

impl Output {
    const GREEN: &'static str = "\x1b[32m";
    const PURPLE: &'static str = "\x1b[35m";
    const RED: &'static str = "\x1b[31m";
    const RESET: &'static str = "\x1b[0m";

    pub fn print_match(r: SearchResult, c: &Config) {
        let mut out = String::new();

        if c.flags & RECURSIVE != 0 {
            write!(out, "{}{}{}:", Output::PURPLE, r.filepath, Output::RESET).unwrap();
        }

        if c.flags & LN_NUMS != 0 {
            write!(out, "{}{}{}:\t", Output::GREEN, r.line_num, Output::RESET).unwrap();
        }

        write!(out, "{}{}{}", Output::RED, r.complement, Output::RESET).unwrap();

        println!("{}", out);
    }
}
