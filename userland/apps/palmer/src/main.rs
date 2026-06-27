use std::{fs::OpenOptions, process::ExitCode};

use ratatui::{
    Terminal,
    crossterm::{
        self,
        event::{Event, KeyEventKind},
        execute,
        terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
    },
    prelude::CrosstermBackend,
};

use crate::{app::App, parse::parse_args};

mod app;
mod components;
mod parse;
mod path;

fn main() -> ExitCode {
    match run() {
        Ok(Some(path)) => {
            println!("{}", path);
            ExitCode::SUCCESS
        }
        Ok(None) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("palmer: {}", e);
            ExitCode::from(2)
        }
    }
}

fn run() -> std::io::Result<Option<String>> {
    let raw_args: Vec<String> = std::env::args().collect();
    let picker_config = parse_args(raw_args);

    let tty = OpenOptions::new().read(true).write(true).open("/dev/tty")?;
    let mut tty_ctrl = tty.try_clone()?;

    enable_raw_mode()?;

    let backend = CrosstermBackend::new(tty);
    let mut terminal = Terminal::new(backend)?;

    let mut app = match picker_config {
        Some(cfg) => App::picker_mode(cfg),
        None => App::new(),
    };

    if !app.picker_mode {
        execute!(tty_ctrl, EnterAlternateScreen)?;
    }

    loop {
        terminal.draw(|frame| app.render(frame))?;
        if let Event::Key(key) = crossterm::event::read()?
            && key.kind == KeyEventKind::Press
            && app.handle_key(key)
        {
            break;
        }
    }

    if !app.picker_mode {
        let _ = execute!(tty_ctrl, LeaveAlternateScreen);
    }
    let _ = disable_raw_mode();

    Ok(app.picked_file())
}
