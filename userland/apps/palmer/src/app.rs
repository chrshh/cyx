use std::path::PathBuf;

use ratatui::{
    Frame,
    crossterm::event::{KeyCode, KeyEvent, KeyModifiers},
    layout::{Constraint, Direction, Layout},
};

use crate::components::{DirList, Preview, Statbar};

pub struct App {
    /* global state  */
    pub cwd: PathBuf,
    /* ui components */
    pub parent: DirList,
    pub current: DirList,
    pub preview: Preview,
    pub statbar: Statbar,
}

impl App {
    pub fn new() -> Self {
        let cwd = std::env::current_dir().unwrap();
        let parent = match cwd.parent() {
            Some(p) => {
                let name = cwd.file_name().and_then(|n| n.to_str());
                DirList::with_highlight(p.to_path_buf(), name)
            }
            None => DirList::empty(),
        };

        let current = DirList::new(&cwd);
        let preview = Preview::from(&cwd, &current);
        let statbar = Statbar::new(&cwd);

        Self {
            cwd,
            parent,
            current,
            preview,
            statbar,
        }
    }

    pub fn render(&mut self, frame: &mut Frame) {
        let vertical = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(2), Constraint::Min(0)])
            .split(frame.area());

        let statbar_area = vertical[0];
        let main_area = vertical[1];

        let columns = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(20),
                Constraint::Percentage(40),
                Constraint::Percentage(40),
            ])
            .split(main_area);

        self.statbar.render(frame, statbar_area);
        self.parent.render(frame, columns[0]);
        self.current.render(frame, columns[1]);
        self.preview.render(frame, columns[2]);
    }

    pub fn handle_key(&mut self, key: KeyEvent) -> bool {
        match (key.modifiers, key.code) {
            /* QUIT */
            (_, KeyCode::Char('q')) | (_, KeyCode::Esc) => return true,
            (KeyModifiers::CONTROL, KeyCode::Char('c')) => return true,

            /* navigate cwd */
            (_, KeyCode::Char('j')) | (_, KeyCode::Down) => self.move_down(),
            (_, KeyCode::Char('k')) | (_, KeyCode::Up) => self.move_up(),

            /* g & G, jump top, jump bottom */
            (_, KeyCode::Char('g')) | (_, KeyCode::Home) => self.jump_top(),
            (_, KeyCode::Char('G')) | (_, KeyCode::End) => self.jump_bottom(),

            /* /* change dir */ */
            (_, KeyCode::Char('h')) | (_, KeyCode::Enter) => self.enter_selected(),
            (_, KeyCode::Char('l')) | (_, KeyCode::Backspace) => self.go_up(),

            /* ignore all else */
            _ => {}
        }
        false
    }

    fn move_down(&mut self) {
        DirList::cursor_down(&mut self.current);
        self.refresh_preview();
    }

    fn move_up(&mut self) {
        DirList::cursor_up(&mut self.current);
        self.refresh_preview();
    }

    fn jump_top(&mut self) {
        DirList::cursor_top(&mut self.current);
    }

    fn jump_bottom(&mut self) {
        DirList::cursor_bottom(&mut self.current);
    }

    fn enter_selected(&mut self) {
        if let Some(entry) = self.current.selected_entry() {
            let new_cwd = self.cwd.join(entry);
            if new_cwd.is_dir() {
                self.cwd = new_cwd;
                self.refresh_all();
            }
        }
    }

    fn go_up(&mut self) {
        if let Some(parent) = self.cwd.parent() {
            self.cwd = parent.to_path_buf();
            self.refresh_all();
        }
    }

    fn refresh_preview(&mut self) {
        self.preview = Preview::from(&self.cwd, &self.current);
    }

    fn refresh_all(&mut self) {
        /* rebuild based on cwd */
        self.parent = match self.cwd.parent() {
            Some(p) => {
                let name = self.cwd.file_name().and_then(|n| n.to_str());
                DirList::with_highlight(p.to_path_buf(), name)
            }
            None => DirList::empty(),
        };
        self.current = DirList::new(&self.cwd);
        self.preview = Preview::from(&self.cwd, &self.current);
        self.statbar = Statbar::new(&self.cwd);
    }
}
