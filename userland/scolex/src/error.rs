use std::cell::Cell;

thread_local! {
    static HAD_ERR: Cell<bool> = const { Cell::new(false) };
}

pub fn error(line: usize, msg: &str) {
    report(line, "", msg);
}

fn report(line: usize, e: &str, msg: &str) {
    println!("[line {} ] Error{}: {}", line, e, msg);
    set_err_flag(true);
}

pub fn read_err_flag() -> bool {
    HAD_ERR.with(|f| f.get())
}

pub fn set_err_flag(f: bool) {
    match f {
        true => HAD_ERR.with(|f| f.set(true)),
        false => HAD_ERR.with(|f| f.set(false)),
    }
}
