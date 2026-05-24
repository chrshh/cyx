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
}

impl Grid {
    pub fn new(cols: usize, rows: usize) -> Self {
        Self {
            cols,
            rows,
            cells: vec![Cell::default(); cols * rows],
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
}
