use std::path::{Path, PathBuf};

use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    widgets::{Block, Borders, Paragraph},
};

#[derive(Debug, Default)]
pub struct Statbar {
    pub current_dir: String,
}

pub fn format_cwd(cwd: &Path) -> String {
    let prefix = "~/";
    let rel_path: PathBuf = cwd.components().skip(3).collect();
    prefix.to_string() + rel_path.to_str().unwrap()
}

impl Statbar {
    pub fn new(current_dir: &Path) -> Self {
        let current_dir = format_cwd(current_dir);
        Self { current_dir }
    }

    pub fn render(&self, frame: &mut Frame, area: Rect) {
        /* bottom border drawn first */
        let block = Block::default().borders(Borders::BOTTOM);
        let inner = block.inner(area);
        frame.render_widget(block, area);

        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(inner);

        frame.render_widget(
            Paragraph::new(self.current_dir.as_str()).left_aligned(),
            chunks[0],
        );
    }
}
