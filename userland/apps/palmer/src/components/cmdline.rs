use std::{collections::HashSet, path::Path};

use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    widgets::{Block, Borders, Paragraph},
};

use crate::path::StringPathExt;

#[derive(Debug, Default)]
pub struct CmdLine {
    pub mode: Option<Mode>,
    pub input: String,
}

#[derive(Debug, Clone, Copy)]
pub enum Mode {
    Grep,
    Find,
}

impl CmdLine {
    pub fn init() -> Self {
        Self {
            mode: None,
            input: "".to_string(),
        }
    }

    pub fn render(&self, frame: &mut Frame, area: Rect) {
        let p = Paragraph::new(self.input.as_str()).block(
            Block::default()
                .borders(Borders::ALL)
                .title(self.get_search_text()),
        );
        frame.render_widget(p, area);
    }

    /* picker-mode variant */
    pub fn render_picker(&self, frame: &mut Frame, area: Rect) {
        let content = format!("{}: {}", self.get_search_text(), self.input);
        let p = Paragraph::new(content).block(
            Block::default()
                .borders(Borders::BOTTOM)
                .border_style(ratatui::style::Style::default().fg(ratatui::style::Color::White)),
        );
        frame.render_widget(p, area);
    }

    pub fn open(&mut self, mode: Mode) {
        self.mode = Some(mode);
        self.input.clear();
    }

    pub fn close(&mut self) {
        self.mode = None;
        self.input.clear();
    }

    pub fn is_open(&self) -> bool {
        self.mode.is_some()
    }

    pub fn get_search_text(&self) -> &str {
        match self.mode {
            Some(Mode::Grep) => "grep",
            Some(Mode::Find) => "find",
            None => "<unknown>",
        }
    }

    pub fn add_char(&mut self, c: char) {
        self.input.push(c);
    }

    pub fn remove_char(&mut self) {
        self.input.pop();
    }

    pub fn render_cmdline_area(&self, percent_x: u16, height: u16, area: Rect) -> Rect {
        let vertical = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(0),
                Constraint::Length(height),
                Constraint::Min(0),
            ])
            .split(area);

        Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ])
            .split(vertical[1])[1]
    }

    /* entry point for cmdline search */
    pub fn search(&mut self, cwd: &Path) -> Vec<String> {
        match self.mode {
            Some(Mode::Grep) => self.search_from_grep(cwd),
            Some(Mode::Find) => self.search_from_find(cwd),
            None => Vec::default(),
        }
    }

    pub fn search_from_grep(&mut self, cwd: &Path) -> Vec<String> {
        let results = cg::search_in(self.input.as_str(), cwd.to_str().unwrap(), 0).unwrap();

        let mut seen: HashSet<String> = HashSet::new();

        results
            .into_iter()
            .filter(|f| seen.insert(f.filepath.clone()))
            .map(|f| f.filepath.pretty(cwd))
            .collect()
    }

    pub fn search_from_find(&mut self, cwd: &Path) -> Vec<String> {
        let results = cfd::find_in(Some(self.input.as_str()), cwd.to_str().unwrap(), 0).unwrap();

        let mut seen: HashSet<String> = HashSet::new();

        results
            .into_iter()
            .filter(|f| {
                seen.insert(f.to_str().unwrap().to_string())
                // && f.file_name().unwrap().to_str().unwrap().starts_with(".")
            })
            .map(|f| f.to_string_lossy().into_owned().pretty(cwd))
            .collect()
    }
}
