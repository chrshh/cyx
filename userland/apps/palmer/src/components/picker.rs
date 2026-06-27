use ratatui::layout::{Constraint, Direction, Layout, Rect};

#[derive(Debug, Copy, Clone)]
pub struct Picker;

impl Picker {
    pub fn new() -> Self {
        Self
    }

    pub fn render_picker_area(&mut self, percent_x: u16, height: u16, area: Rect) -> Rect {
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
}
