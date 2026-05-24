use std::mem;

#[derive(Copy, Clone, Debug)]
pub struct Cell {
    pub ch: char,
    pub fg: [u8; 3],
    pub bg: [u8; 3],
}

impl Default for Cell {
    fn default() -> Self {
        Self {
            ch: ' ',
            fg: [0xe0, 0xe0, 0xe0],
            bg: [0x1e, 0x1e, 0x1e],
        }
    }
}

impl Cell {
    pub fn default_fg(&mut self) {
        self.fg = [0xe0, 0xe0, 0xe0];
    }
    pub fn default_bg(&mut self) {
        self.bg = [0x1e, 0x1e, 0x1e];
    }
}

pub struct Grid {
    pub cols: usize,
    pub rows: usize,
    pub cells: Vec<Cell>,
    pub alt_cells: Option<Vec<Cell>>,
    pub saved_cursor: Option<(usize, usize)>,
}

impl Grid {
    pub fn new(cols: usize, rows: usize) -> Self {
        Self {
            cols,
            rows,
            cells: vec![Cell::default(); cols * rows],
            alt_cells: None,
            saved_cursor: None,
        }
    }

    pub fn resize(&mut self, cols: usize, rows: usize) {
        self.cols = cols;
        self.rows = rows;
        self.cells = vec![Cell::default(); cols * rows];
    }

    pub fn put(&mut self, col: usize, row: usize, ch: char) {
        if col < self.cols && row < self.rows {
            self.cells[row * self.cols + col].ch = ch;
        }
    }

    pub fn write_str(&mut self, col: usize, row: usize, s: &str) {
        for (i, ch) in s.chars().enumerate() {
            self.put(col + i, row, ch);
        }
    }

    pub fn clear_range(&mut self, start: usize, end: usize) {
        let end = end.min(self.cells.len());
        for cell in &mut self.cells[start..end] {
            *cell = Cell::default();
        }
    }

    pub fn scroll_up(&mut self) {
        self.cells.copy_within(self.cols.., 0);
        let last_start = (self.rows - 1) * self.cols;
        for cell in &mut self.cells[last_start..] {
            *cell = Cell::default();
        }
    }

    pub fn enter_alt(&mut self, cursor_col: usize, cursor_row: usize) {
        if self.alt_cells.is_some() {
            return; // on alt screen
        }

        let blank = vec![Cell::default(); self.cols * self.rows];
        let main = mem::replace(&mut self.cells, blank);
        self.alt_cells = Some(main);
        self.saved_cursor = Some((cursor_col, cursor_row));
    }

    pub fn exit_alt(&mut self) -> Option<(usize, usize)> {
        let main = self.alt_cells.take()?;
        self.cells = main;
        self.saved_cursor.take()
    }
}
