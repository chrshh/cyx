use std::{
    fs::read_to_string,
    io::{self, Write},
    process::exit,
};

use crate::{
    error::{read_err_flag, set_err_flag},
    scanner::Scanner,
    token::{GenericToken, Literal},
};

mod error;
mod scanner;
mod token;
mod token_type;

pub type L = Literal;
pub type Token = GenericToken<L>;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 2 {
        println!("Usage: scolex [script]");
        exit(64);
    } else if args.len() == 2 {
        run_file(&args[1]);
    } else {
        run_prompt();
    }
    println!("Hello, world!");
}

// Script
fn run_file(path: &String) {
    let file = read_to_string(path).unwrap();
    run(&file);

    if read_err_flag() {
        exit(65);
    }
}

// REPL
fn run_prompt() {
    loop {
        print!("scolex ~ ");
        io::stdout().flush().unwrap();

        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        if line.is_empty() || line == "exit" {
            break;
        }

        run(&line);
        set_err_flag(false);
    }
}

/* entry point for running a scolex source file */
fn run(source: &str) {
    let scanner = Scanner::new(source);

    let tokens: Vec<Token> = scanner.collect();
    let _ = tokens.into_iter().map(|t| print!("{:?}", t));
}
