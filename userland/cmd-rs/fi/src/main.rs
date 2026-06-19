#![recursion_limit = "512"]
use std::{
    env::args,
    process::ExitCode,
    sync::{Arc, atomic::AtomicBool},
    time::Instant,
};

use crate::{error::CGrepError, parse::parse_args, search::search};

mod error;
mod flags;
mod parse;
mod search;
mod thread;
mod writer;

pub static MATCH_FOUND: AtomicBool = AtomicBool::new(false);

fn main() -> ExitCode {
    match run() {
        Ok(true) => ExitCode::SUCCESS,
        Ok(false) => ExitCode::from(1),
        Err(e) => {
            eprintln!("{e}");
            ExitCode::from(2)
        }
    }
}

fn run() -> Result<bool, CGrepError> {
    let start = Instant::now();
    let raw_args: Vec<String> = args().collect();
    let cfg = parse_args(&raw_args[1..])?;
    let cfg = Arc::new(cfg);
    search(cfg, start)
}
