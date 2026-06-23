use ratatui::{Frame, buffer::Buffer, layout::Rect, widgets::Paragraph};

#[derive(Debug)]
pub struct Statbar {
    pub welcome_msg: String,
}

impl Statbar {
    pub fn new() -> Self {
        let welcome_msg = "palmer v0.0.1".to_string();
        Self { welcome_msg }
    }

    pub fn render(&self, frame: &mut Frame, area: Rect) {
        let paragraph = Paragraph::new(self.welcome_msg.clone());
        frame.render_widget(paragraph, area);
    }
}
