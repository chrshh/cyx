use ratatui::{DefaultTerminal, crossterm};

use crate::app::App;

mod app;
mod components;
mod path;

fn main() -> Result<(), ()> {
    ratatui::run(app).unwrap();
    Ok(())
}

fn app(terminal: &mut DefaultTerminal) -> std::io::Result<()> {
    let mut app = App::new();
    loop {
        terminal.draw(|frame| app.render(frame))?;
        if let crossterm::event::Event::Key(key) = crossterm::event::read()?
            && key.kind == crossterm::event::KeyEventKind::Press
            && app.handle_key(key)
        {
            // returns true -> quit
            break Ok(());
        }
    }
}
