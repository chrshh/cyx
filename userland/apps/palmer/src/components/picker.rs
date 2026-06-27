use ratatui::{
    crossterm::event::{KeyCode, KeyEvent, KeyModifiers},
    layout::{Constraint, Direction, Layout, Rect},
};

use crate::app::App;
#[derive(Debug, Copy, Clone)]
pub struct Picker;

impl Picker {
    pub fn new() -> Self {
        Self
    }

    pub fn render_picker_area(&mut self, percent_x: u16, height: u16, area: Rect) -> Rect {
        let vertical = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(0),
                Constraint::Length(height),
                Constraint::Min(0),
            ])
            .split(area);

        Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ])
            .split(vertical[1])[1]
    }
}

pub fn handle_picker_key(key: KeyEvent, app: &mut App) -> bool {
    /* mode 0: insert (default) */
    /* mode 1: normal mode for nav */
    match app.picker_state {
        0 => handle_insert_mode_picker(key, app),
        1 => handle_normal_mode_picker(key, app),
        _ => false,
    }
}

pub fn handle_insert_mode_picker(key: KeyEvent, app: &mut App) -> bool {
    match (key.modifiers, key.code) {
        /* QUIT */
        (KeyModifiers::CONTROL, KeyCode::Char('c')) => return true,

        /* enter normal mode */
        (_, KeyCode::Esc) => app.picker_state = 1,

        (_, KeyCode::Backspace) => {
            app.cmdline.remove_char();
            app.picker_search();
        }

        /* navigate cwd */
        (_, KeyCode::Down) => app.move_down(),
        (_, KeyCode::Up) => app.move_up(),

        /* enter chars */
        _ => {
            app.cmdline.add_char(key.code.as_char().unwrap());
            app.picker_search();
        }
    }
    false
}

pub fn handle_normal_mode_picker(key: KeyEvent, app: &mut App) -> bool {
    match (key.modifiers, key.code) {
        /* QUIT */
        (KeyModifiers::CONTROL, KeyCode::Char('c')) => return true,
        (_, KeyCode::Esc) => return true,

        /* navigate cwd */
        (_, KeyCode::Char('j')) | (_, KeyCode::Down) => app.move_down(),
        (_, KeyCode::Char('k')) | (_, KeyCode::Up) => app.move_up(),

        /* g & G, jump top, jump bottom */
        (_, KeyCode::Char('g')) | (_, KeyCode::Home) => app.jump_top(),
        (_, KeyCode::Char('G')) | (_, KeyCode::End) => app.jump_bottom(),

        /* change dir */
        (_, KeyCode::Char('l')) => app.enter_selected(),
        (_, KeyCode::Char('h')) | (_, KeyCode::Backspace) => app.go_up(),

        (_, KeyCode::Char('.')) => app.toggle_hidden(),

        (_, KeyCode::Enter) => {
            if let Some(entry) = app.current.selected_entry() {
                let file = app.cwd.join(entry);
                app.picked = Some(file);
                return true;
            }
        }

        /* enter insert mode */
        (_, KeyCode::Char('i')) => app.picker_state = 0,

        /* ignore all else */
        _ => {}
    }
    false
}
