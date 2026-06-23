use std::path::PathBuf;

use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
};

use crate::components::{DirList, Statbar};

pub struct App {
    pub parent: DirList,
    pub current: DirList,
    pub statbar: Statbar,
}

impl App {
    pub fn new() -> Self {
        let home = PathBuf::from(std::env::var("HOME").unwrap());
        let parent_path = home.parent().unwrap_or(&home).to_path_buf();

        Self {
            parent: DirList::new(parent_path),
            current: DirList::new(home),
            statbar: Statbar::new(),
        }
    }

    pub fn render(&self, frame: &mut Frame) {
        let vertical = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(1), Constraint::Min(0)])
            .split(frame.area());

        let statbar_area = vertical[0];
        let main_area = vertical[1];

        let columns = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(main_area);

        self.statbar.render(frame, statbar_area);
        self.parent.render(frame, columns[0]);
        self.current.render(frame, columns[1]);
    }
}
