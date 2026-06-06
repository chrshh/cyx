use std::fmt::Write;

use crate::{
    input::{Argv, LN_NUMS, RECURSIVE},
    search::Match,
};

#[derive(Debug, Default)]
pub struct Output;

impl Output {
    pub fn build_output(&self, m: &mut Match, a: &Argv) {
        match a.flags {
            0 => self.build_without_flags(m, a.flags),
            _ => self.build_with_flags(m, a.flags),
        }
    }

    pub fn build_with_flags(&self, m: &mut Match, a: u32) {
        let mut out = String::new();

        if a & RECURSIVE != 0 {
            write!(out, "{}:", m.filename).unwrap();
        }

        if a & LN_NUMS != 0 {
            write!(out, "{}:\t", m.line_num).unwrap();
        }

        write!(out, "{}", m.complement).unwrap();

        println!("{}", out);
    }

    pub fn build_without_flags(&self, m: &mut Match, a: u32) {
        let mut out = String::new();
        write!(out, "{}", m.complement).unwrap();
        println!("{}", out);
    }
}
