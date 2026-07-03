use std::{
    fs::read_to_string,
    io::{self, Write},
    process::exit,
};

use crate::{
    error::{
        read_err_flag, read_runtime_err_flag, set_err_flag,
    },
    interpreter::Interpreter,
    parser::{Parser, Stmt},
    scanner::Scanner,
    token::{GenericToken, Literal, Tokens},
};

mod ast;
mod ast_printer;
mod environment;
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

    // `-d` / `--debug` dumps the token stream and parsed AST.
    let debug =
        args.iter().any(|a| a == "-d" || a == "--debug");
    let scripts: Vec<&String> = args
        .iter()
        .skip(1)
        .filter(|a| !a.starts_with('-'))
        .collect();

    match scripts.as_slice() {
        [] => run_prompt(debug),
        [path] => run_file(path, debug),
        _ => {
            println!("Usage: scolex [-d] [script]");
            exit(64);
        }
    }
}

// Script
fn run_file(path: &str, debug: bool) {
    let file = read_to_string(path).unwrap();
    run(&file, debug);

    // 65: compile/syntax error. 70: runtime error. Mirrors jlox.
    if read_err_flag() {
        exit(65);
    }
    if read_runtime_err_flag() {
        exit(70);
    }
}

// REPL
fn run_prompt(debug: bool) {
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

        run(&line, debug);
        set_err_flag(false);
    }
}

/* entry point for running a scolex source file */
fn run(source: &str, debug: bool) {
    let mut scanner = Scanner::new(source);
    let tokens: Vec<Token> = scanner.scan_tokens();

    if debug {
        println!("=== tokens ===");
        print!("{}", Tokens(tokens.clone()));
    }

    let mut parser = Parser::new(tokens);
    let statements: Vec<Stmt> = parser.parse();

    // Don't try to run a program that didn't parse cleanly.
    if read_err_flag() {
        return;
    }

    if debug {
        println!("=== ast ===");
        for stmt in &statements {
            println!("{}", ast_printer::print_stmt(stmt));
        }
        println!("=== output ===");
    }

    let mut interpreter = Interpreter::new();
    interpreter.interpret(statements);
}
