use std::path::PathBuf;

use crate::path::PathBufExt;

use ratatui::{
    Frame,
    layout::Rect,
    style::{Modifier, Style},
    widgets::{Block, Borders, List, ListItem, ListState},
};

#[derive(Debug, Default)]
pub struct DirList {
    pub entries: Vec<String>,
    pub selected: ListState,
}

impl DirList {
    /* initalizer for cwd */
    pub fn new(path: &PathBuf) -> Self {
        let entries = path.get_all();
        let mut selected = ListState::default();
        selected.select(Some(0));
        Self { entries, selected }
    }

    /* initializer for parent dir */
    pub fn with_highlight(path: PathBuf, name: Option<&str>) -> Self {
        if name.unwrap().is_empty() {
            println!("ok");
        }
        let entries = path.get_all();
        let mut selected = ListState::default();
        let idx = name.and_then(|n| entries.iter().position(|e| e == n));
        selected.select(idx);

        Self { entries, selected }
    }

    /* initalizer only used as a fallback */
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

    /* max cursor: 0 , min cursor: entries.len - 1 */
    pub fn cursor_up(&mut self) {
        let pos = self.selected.selected().unwrap_or(0);
        if pos > 0 {
            self.selected.select(Some(pos - 1));
        }
    }

    pub fn cursor_down(&mut self) {
        let pos = self.selected.selected().unwrap_or(0);
        if pos + 1 < self.entries.len() {
            self.selected.select(Some(pos + 1));
        }
    }

    pub fn cursor_top(&mut self) {
        self.selected.select_first();
    }

    pub fn cursor_bottom(&mut self) {
        self.selected.select_last();
    }

    pub fn selected_entry(&mut self) -> Option<&str> {
        self.selected
            .selected()
            .and_then(|pos| self.entries.get(pos).map(|s| s.as_str()))
    }
}
