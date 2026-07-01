use std::{
    fs::read_to_string,
    io::{self, Write},
    process::exit,
};

use crate::{
    ast::Expr,
    ast_printer::print,
    error::{read_err_flag, set_err_flag},
    interpreter::Interpreter,
    parser::Parser,
    scanner::Scanner,
    token::{GenericToken, Literal},
};

mod ast;
mod ast_printer;
mod error;
mod interpreter;
mod parser;
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
        io::stdin()
            .read_line(&mut line)
            .expect("failed to read line");
        if line.is_empty() || line == "exit" {
            break;
        }

        run(&line);
        set_err_flag(false);
    }
}

/* entry point for running a scolex source file */
fn run(source: &str) {
    let mut scanner = Scanner::new(source);
    let tokens: Vec<Token> = scanner.scan_tokens();
    let mut parser = Parser::new(tokens);
    let expr = parser.parse();
    let interpreter = Interpreter::new();
    interpreter.interpret(expr);

    if read_err_flag() {
        exit(65);
    }
}
