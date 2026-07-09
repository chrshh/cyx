mod commands;
mod consts;
mod dbg;
mod editor;
mod history;
mod input;
mod lsp;
mod motions;
mod palmer;
mod rows;
mod search;
mod statbar;
mod syntax;
mod terminal;

use editor::Editor;
use terminal::enable_raw_mode;

fn main() {
    enable_raw_mode();

    /*
     * read debug logs
     * "tail -f /tmp/asemics.log"
     */
    let mut editor = Editor::new();

    let args: Vec<String> = std::env::args_os()
        .map(|a| a.to_string_lossy().into_owned())
        .collect();
    if args.len() >= 2 {
        let filename = args[1].clone();
        editor.open(&filename);
    }

    loop {
        editor.refresh_screen();
        editor.process_key();
    }
}
