use std::path::Path;

use ratatui::{
    Frame,
    layout::Rect,
    widgets::{Block, Borders, Paragraph},
};

use crate::components::DirList;

pub struct Preview {
    content: String,
}

impl Preview {
    pub fn from(cwd: &Path, curr_item: &DirList) -> Self {
        let selected_entry = cwd
            .join(&curr_item.entries[curr_item.selected.selected().unwrap_or_default()])
            .to_string_lossy()
            .into_owned();

        Self {
            content: selected_entry,
        }
    }

    pub fn render(&mut self, frame: &mut Frame, area: Rect) {
        let c = Paragraph::new(self.content.as_str()).block(Block::default().borders(Borders::ALL));

        frame.render_widget(c, area);
    }
}
