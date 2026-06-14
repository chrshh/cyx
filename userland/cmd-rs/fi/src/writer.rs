use crate::{
    flags::{LN_NUMS, RECURSIVE},
    parse::Config,
    search::SearchResult,
};
use std::fmt::Write;

#[derive(Debug, Default)]
pub struct Output;

impl Output {
    pub fn print_match(r: SearchResult, c: &Config) {
        let mut out = String::new();

        if c.flags & RECURSIVE != 0 {
            write!(out, "{}:", r.filepath).unwrap();
        }

        if c.flags & LN_NUMS != 0 {
            write!(out, "{}:\t", r.line_num).unwrap();
        }

        write!(out, "{}", r.complement).unwrap();

        println!("{}", out);
    }
}
