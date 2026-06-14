use std::{env::args, process::ExitCode};

use crate::{error::CGrepError, parse::parse_args, search::search};

mod error;
mod flags;
mod parse;
mod search;
mod writer;

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
    let raw_args: Vec<String> = args().collect();
    let cfg = parse_args(&raw_args[1..])?;
    search(&cfg)
}
