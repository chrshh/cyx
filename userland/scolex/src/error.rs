use std::cell::Cell;

use crate::{Token, token_type::TokenType};

thread_local! {
    static HAD_ERR: Cell<bool> = const { Cell::new(false) };
    static HAD_RUNTIME_ERR: Cell<bool> = const { Cell::new(false) };
}

/// A runtime (interpretation-time) error. Carries the token it occurred
/// at so we can report the right line, mirroring jlox's `RuntimeError`.
#[derive(Debug, Clone)]
pub struct RuntimeError {
    pub token: Token,
    pub message: String,
}

impl RuntimeError {
    pub fn new(token: Token, message: impl Into<String>) -> Self {
        Self {
            token,
            message: message.into(),
        }
    }
}

/// Scanner-level error: we only have a line number, no token.
pub fn error(line: usize, msg: &str) {
    report(line, "", msg);
}

/// Parser-level error: we have the offending token, so we can point at
/// exactly where it went wrong (`" at end"` or `" at '<lexeme>'"`).
pub fn error_at_token(token: &Token, msg: &str) {
    if token.token_type == TokenType::Eof {
        report(token.line, " at end", msg);
    } else {
        report(token.line, &format!(" at '{}'", token.lexeme), msg);
    }
}

/// Prints a runtime error and trips the runtime-error flag (exit 70).
pub fn runtime_error(err: &RuntimeError) {
    eprintln!("{}\n[line {}]", err.message, err.token.line);
    set_runtime_err_flag(true);
}

fn report(line: usize, location: &str, msg: &str) {
    eprintln!("[line {}] Error{}: {}", line, location, msg);
    set_err_flag(true);
}

pub fn read_err_flag() -> bool {
    HAD_ERR.with(|f| f.get())
}

pub fn set_err_flag(f: bool) {
    HAD_ERR.with(|c| c.set(f));
}

pub fn read_runtime_err_flag() -> bool {
    HAD_RUNTIME_ERR.with(|f| f.get())
}

pub fn set_runtime_err_flag(f: bool) {
    HAD_RUNTIME_ERR.with(|c| c.set(f));
}
