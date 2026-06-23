use std::{fs::read_dir, path::PathBuf};

use ratatui::{
    Frame,
    layout::Rect,
    widgets::{Block, Borders, List, ListItem},
};

#[derive(Debug)]
pub struct DirList {
    pub path: PathBuf,
    pub entries: Vec<String>,
}

impl DirList {
    pub fn new(path: PathBuf) -> Self {
        let entries = read_dir(&path)
            .map(|rd| {
                rd.filter_map(|e| e.ok())
                    .map(|e| e.file_name().to_string_lossy().into_owned())
                    .collect()
            })
            .unwrap_or_default();
        Self { path, entries }
    }

    pub fn render(&self, frame: &mut Frame, area: Rect) {
        let items: Vec<ListItem> = self
            .entries
            .iter()
            .map(|e| ListItem::new(e.as_str()))
            .collect();

        let title = self.path.to_string_lossy().into_owned();
        let list = List::new(items).block(Block::default().title(title).borders(Borders::ALL));

        frame.render_widget(list, area);
    }
}
