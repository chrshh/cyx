use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
};

use crate::components::{DirList, Statbar};

pub struct App {
    /* ui components */
    pub parent: DirList,
    pub current: DirList,
    pub preview: DirList,
    pub statbar: Statbar,
}

impl App {
    pub fn new() -> Self {
        let current_dir = std::env::current_dir().unwrap();
        let parent_path = current_dir.parent().unwrap_or(&current_dir).to_path_buf();

        Self {
            parent: DirList::new(&parent_path),
            current: DirList::new(&current_dir),
            preview: DirList::new(&current_dir),
            statbar: Statbar::new(&current_dir),
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
}
