use std::path::PathBuf;

use crate::path::PathBufExt;

use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Padding},
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

    pub fn from_search(results: Vec<String>) -> Self {
        let mut selected = ListState::default();
        selected.select(Some(0));
        if results.is_empty() {
            panic!("entries from search are empty")
        }
        Self {
            entries: results,
            selected,
        }
    }

    pub fn render(&mut self, frame: &mut Frame, area: Rect) {
        let items: Vec<ListItem> = self
            .entries
            .iter()
            .map(|e| {
                // Check if it's a directory (simple check based on trailing slash,
                // or adapt this to match your specific entry struct logic)
                let is_dir = e.ends_with('/') || !e.contains('.');

                let line = if is_dir {
                    Line::from(vec![
                        Span::styled(" ", Style::default().fg(Color::LightYellow)),
                        Span::raw(e.as_str()),
                    ])
                } else {
                    Line::from(vec![
                        Span::styled(" ", Style::default().fg(Color::White)),
                        Span::raw(e.as_str()),
                    ])
                };

                ListItem::new(line)
            })
            .collect();

        let list = List::new(items)
            .block(
                Block::default()
                    .borders(Borders::RIGHT)
                    .border_style(Style::default().fg(Color::DarkGray))
                    .padding(Padding::left(2)),
            )
            .highlight_style(
                Style::default()
                    .bg(Color::White)
                    .fg(Color::Black)
                    .add_modifier(Modifier::BOLD),
            );

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
