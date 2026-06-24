use std::{
    fs::read_dir,
    path::{Path, PathBuf},
};

use ratatui::{
    Frame,
    layout::Rect,
    style::{Modifier, Style},
    widgets::{Block, Borders, List, ListItem, ListState},
};

#[derive(Debug)]
pub struct DirList {
    pub entries: Vec<String>,
    pub selected: ListState,
}

impl DirList {
    pub fn new(path: &Path) -> Self {
        let entries: Vec<String> = read_dir(path)
            .map(|rd| {
                rd.filter_map(|e| e.ok())
                    .map(|e| e.file_name().to_string_lossy().into_owned())
                    .collect()
            })
            .unwrap_or_default();

        let mut selected = ListState::default();
        selected.select(Some(0));
        Self { entries, selected }
    }

    pub fn with_highlight(path: PathBuf, name: Option<&str>) -> Self {
        if name.unwrap().is_empty() {
            println!("ok");
        }
        let entries: Vec<String> = read_dir(&path)
            .map(|rd| {
                rd.filter_map(|e| e.ok().map(|e| e.file_name().to_string_lossy().into_owned()))
                    .collect()
            })
            .unwrap_or_default();

        let mut selected = ListState::default();
        let idx = name.and_then(|n| entries.iter().position(|e| e == n));
        selected.select(idx);

        Self { entries, selected }
    }

    pub fn empty() -> Self {
        Self {
            entries: Vec::default(),
            selected: ListState::default(),
        }
    }

    pub fn render(&mut self, frame: &mut Frame, area: Rect) {
        let items: Vec<ListItem> = self
            .entries
            .iter()
            .map(|e| ListItem::new(e.as_str()))
            .collect();

        let list = List::new(items)
            .block(Block::default().borders(Borders::ALL))
            .highlight_style(Style::default().add_modifier(Modifier::REVERSED));

        frame.render_stateful_widget(list, area, &mut self.selected);
    }
}
