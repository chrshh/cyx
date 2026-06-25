use std::{env::args, process::ExitCode};

fn main() -> ExitCode {
    let raw_args: Vec<String> = args().collect();
    match cg::run(&raw_args[1..]) {
        Ok(true) => ExitCode::SUCCESS,
        Ok(false) => ExitCode::from(1),
        Err(e) => {
            eprintln!("{e}");
            ExitCode::from(2)
        }
    }
}
