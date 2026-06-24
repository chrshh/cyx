use std::path::PathBuf;

use ratatui::{
    Frame,
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
}
