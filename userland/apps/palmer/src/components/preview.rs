use std::{fs::read_to_string, path::Path};

use ratatui::{Frame, layout::Rect, widgets::Paragraph};

use crate::{
    components::DirList,
    path::{PathBufExt, StringPathExt},
};

#[derive(Debug, Default)]
pub enum Preview {
    File(String),
    Dir(Vec<String>),
    #[default]
    None,
}

impl Preview {
    /* handle entry types to render */
    pub fn from(cwd: &Path, curr_item: &DirList) -> Self {
        let idx = curr_item.selected.selected();
        /* return empty preview for no index */
        if idx.is_none() {
            return Preview::None;
        }
        let selected_entry = cwd.join(&curr_item.entries[idx.unwrap_or_default()]);

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

    pub fn render(&mut self, frame: &mut Frame, area: Rect, show_hidden: bool) {
        let text: String = match self {
            Preview::File(s) => s.clone(),
            Preview::Dir(entries) => entries
                .iter()
                .filter(|s| !s.is_hidden() || show_hidden)
                .cloned()
                .collect::<Vec<_>>()
                .join("\n"),
            Preview::None => String::new(),
        };

        let c = Paragraph::new(text);
        frame.render_widget(c, area);
    }
}
