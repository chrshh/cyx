use std::{fs::read_to_string, path::Path};

use ratatui::{Frame, layout::Rect, widgets::Paragraph};

use crate::{components::DirList, path::PathBufExt};

#[derive(Default)]
pub enum Preview {
    File(String),
    Dir(Vec<String>),
    #[default]
    None,
}

impl Preview {
    /* handle entry types to render */
    pub fn from(cwd: &Path, curr_item: &DirList) -> Self {
        let idx = curr_item.selected.selected().unwrap_or_default();
        let selected_entry = cwd.join(&curr_item.entries[idx]);

        if selected_entry.is_file() {
            match read_to_string(&selected_entry) {
                Ok(text) => Preview::File(text),
                Err(_) => Preview::None,
            }
        } else if selected_entry.is_dir() {
            let entries = selected_entry.get_all();
            Preview::Dir(entries)
        } else {
            Preview::None
        }
    }

    pub fn render(&mut self, frame: &mut Frame, area: Rect) {
        let text = match self {
            Preview::File(s) => s.clone(),
            Preview::Dir(entries) => entries.join("\n"),
            Preview::None => String::new(),
        };

        let c = Paragraph::new(text);
        frame.render_widget(c, area);
    }
}
