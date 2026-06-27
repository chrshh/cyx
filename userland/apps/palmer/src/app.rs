use std::path::PathBuf;

use ratatui::{
    Frame,
    crossterm::event::{KeyCode, KeyEvent, KeyModifiers},
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, Clear},
};

use crate::{
    components::{CmdLine, DirList, Picker, Preview, Statbar, cmdline::Mode},
    parse::RunConfig,
};

#[derive(Debug)]
pub struct App {
    /* global state  */
    pub cwd: PathBuf,
    pub show_hidden: bool,
    /* ui components */
    pub parent: DirList,
    pub current: DirList,
    pub preview: Preview,
    pub statbar: Statbar,
    pub cmdline: CmdLine,
    /* plugin fields */
    pub picker: Option<Picker>,
    pub picked: Option<PathBuf>,
    pub picker_mode: bool,
}

impl App {
    pub fn new() -> Self {
        let cwd = std::env::current_dir().unwrap();
        let parent = match cwd.parent() {
            Some(p) => {
                let name = cwd.file_name().and_then(|n| n.to_str());
                DirList::with_highlight(p.to_path_buf(), name)
            }
            None => DirList::empty(),
        };

        let show_hidden = false;
        let current = DirList::new(&cwd);
        let preview = Preview::from(&cwd, &current);
        let statbar = Statbar::new(&cwd);
        let cmdline = CmdLine::init();
        let picker = None;
        let picked = None;
        let picker_mode = false;

        Self {
            cwd,
            show_hidden,
            parent,
            current,
            preview,
            statbar,
            cmdline,
            picker,
            picked,
            picker_mode,
        }
    }

    pub fn picker_mode(args: RunConfig) -> Self {
        let cwd = if args.provided_dir.to_str().unwrap().is_empty() {
            std::env::current_dir().unwrap_or_default()
        } else {
            args.provided_dir
        };

        let show_hidden = false;
        let parent = DirList::empty();
        let current = DirList::new(&cwd);
        let preview = Preview::from(&cwd, &current);
        let statbar = Statbar::empty();
        let mut cmdline = CmdLine::init();
        cmdline.mode = Some(Mode::Find);
        let picker = Picker::new();
        let picked = None;
        let picker_mode = true;

        Self {
            cwd,
            show_hidden,
            current,
            parent,
            preview,
            statbar,
            cmdline,
            picker: Some(picker),
            picked,
            picker_mode,
        }
    }

    /*
     * getter method only accessed when pressing
     * ENTER to return file path name
     * */
    pub fn picked_file(&self) -> Option<String> {
        self.picked
            .as_ref()
            .map(|p| p.to_string_lossy().into_owned())
    }

    pub fn render(&mut self, frame: &mut Frame) {
        if self.picker_mode {
            self.render_picker_mode(frame)
        } else {
            let vertical = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Length(1), Constraint::Min(1)])
                .split(frame.area());

            let statbar_area = vertical[0];
            let main_area = vertical[1];

            let columns = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([
                    Constraint::Percentage(20),
                    Constraint::Percentage(40),
                    Constraint::Percentage(40),
                ])
                .split(main_area);

            self.statbar.render(frame, statbar_area);
            self.parent.render(frame, columns[0], self.show_hidden);
            self.current.render(frame, columns[1], self.show_hidden);
            self.preview.render(frame, columns[2], self.show_hidden);

            if self.cmdline.is_open() {
                let area = self.cmdline.render_cmdline_area(60, 3, frame.area());
                frame.render_widget(Clear, area);
                self.cmdline.render(frame, area);
            }
        }
    }

    pub fn render_picker_mode(&mut self, frame: &mut Frame) {
        let picker_area = self
            .picker
            .unwrap()
            .render_picker_area(50, 25, frame.area());

        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::White));

        frame.render_widget(Clear, picker_area);
        frame.render_widget(block.clone(), picker_area);
        let inner = block.inner(picker_area);

        let vertical = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(2), Constraint::Min(1)])
            .split(inner);

        let cmdline_area = vertical[0];
        let main_area = vertical[1];

        let columns = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(main_area);

        self.cmdline.render_picker(frame, cmdline_area);
        self.current.render(frame, columns[0], self.show_hidden);
        self.preview.render(frame, columns[1], self.show_hidden);
    }

    pub fn handle_key(&mut self, key: KeyEvent) -> bool {
        /* intercept keystrokes if cmdline is open */
        if self.cmdline.mode.is_some() {
            return self.handle_cmdline_key(key);
        }
        match (key.modifiers, key.code) {
            /* QUIT */
            (_, KeyCode::Char('q')) => return true,
            (KeyModifiers::CONTROL, KeyCode::Char('c')) => return true,

            /* refresh (to clear search results) */
            (_, KeyCode::Esc) => self.refresh_all(),

            /* navigate cwd */
            (_, KeyCode::Char('j')) | (_, KeyCode::Down) => self.move_down(),
            (_, KeyCode::Char('k')) | (_, KeyCode::Up) => self.move_up(),

            /* g & G, jump top, jump bottom */
            (_, KeyCode::Char('g')) | (_, KeyCode::Home) => self.jump_top(),
            (_, KeyCode::Char('G')) | (_, KeyCode::End) => self.jump_bottom(),

            /* change dir */
            (_, KeyCode::Char('l')) | (_, KeyCode::Enter) => self.enter_selected(),
            (_, KeyCode::Char('h')) | (_, KeyCode::Backspace) => self.go_up(),

            (_, KeyCode::Char('.')) => self.toggle_hidden(),

            /* cmdline keybinds */
            (_, KeyCode::Char('s')) => self.cmdline.open(Mode::Find),
            (_, KeyCode::Char('S')) => self.cmdline.open(Mode::Grep),

            /* ignore all else */
            _ => {}
        }
        false
    }

    pub fn handle_cmdline_key(&mut self, key: KeyEvent) -> bool {
        match (key.modifiers, key.code) {
            /* QUIT */
            (_, KeyCode::Char('q')) => return true,
            (KeyModifiers::CONTROL, KeyCode::Char('c')) => return true,

            (_, KeyCode::Esc) => self.cmdline.close(),
            (_, KeyCode::Enter) => self.search(),

            (_, KeyCode::Backspace) => self.cmdline.remove_char(),

            _ => self.cmdline.add_char(key.code.as_char().unwrap()),
        }
        false
    }

    fn move_down(&mut self) {
        DirList::cursor_down(&mut self.current);
        self.refresh_preview();
    }

    fn move_up(&mut self) {
        DirList::cursor_up(&mut self.current);
        self.refresh_preview();
    }

    fn jump_top(&mut self) {
        DirList::cursor_top(&mut self.current);
    }

    fn jump_bottom(&mut self) {
        DirList::cursor_bottom(&mut self.current);
    }

    fn enter_selected(&mut self) {
        if let Some(entry) = self.current.selected_entry() {
            let new_cwd = self.cwd.join(entry);
            if new_cwd.is_dir() {
                self.cwd = new_cwd;
                self.refresh_all();
            }
        }
    }

    fn go_up(&mut self) {
        if let Some(parent) = self.cwd.parent() {
            self.cwd = parent.to_path_buf();
            self.refresh_all();
        }
    }

    fn toggle_hidden(&mut self) {
        self.show_hidden = !self.show_hidden;
        self.refresh_all();
    }

    fn refresh_preview(&mut self) {
        self.preview = Preview::from(&self.cwd, &self.current);
    }

    fn refresh_statbar(&mut self) {
        self.statbar = Statbar::from(&self.cwd, &self.cmdline);
    }

    fn refresh_all(&mut self) {
        /* rebuild based on cwd */
        self.parent = match self.cwd.parent() {
            Some(p) => {
                let name = self.cwd.file_name().and_then(|n| n.to_str());
                DirList::with_highlight(p.to_path_buf(), name)
            }
            None => DirList::empty(),
        };
        self.current = DirList::new(&self.cwd);
        self.preview = Preview::from(&self.cwd, &self.current);
        self.statbar = Statbar::new(&self.cwd);
    }

    fn search(&mut self) {
        self.current = DirList::from_search(self.cmdline.search(&self.cwd));
        self.refresh_statbar();
        self.cmdline.close();
        self.refresh_preview();
    }
}
