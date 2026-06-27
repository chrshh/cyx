use crate::{components::CmdLine, path::PathExt};
use std::path::Path;

use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Padding, Paragraph},
};

#[derive(Debug, Default)]
pub struct Statbar {
    pub content: String,
}

impl Statbar {
    pub fn new(cwd: &Path) -> Self {
        Self {
            content: cwd.pretty(),
        }
    }

    pub fn from(cwd: &Path, cli: &CmdLine) -> Self {
        if cli.mode.is_none() {
            return Self {
                content: cwd.pretty(),
            };
        }
        let content = cwd.pretty() + " | " + cli.get_search_text() + ": " + cli.input.as_str();
        Self { content }
    }

    pub fn empty() -> Self {
        Self {
            content: String::new(),
        }
    }

    pub fn render(&self, frame: &mut Frame, area: Rect) {
        let block = Block::default().padding(Padding::horizontal(1));
        let inner = block.inner(area);
        frame.render_widget(block, area);

        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(inner);

        frame.render_widget(
            Paragraph::new(self.content.as_str())
                .style(Style::default().fg(Color::Cyan))
                .left_aligned(),
            chunks[0],
        );
    }
}
