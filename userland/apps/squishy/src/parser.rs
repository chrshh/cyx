use crate::grid::{Cell, Grid};

// add <Cell> in later for coloring
pub struct Cursor {
    pub col: usize,
    pub row: usize,
    pub cell: Cell,
}

pub struct Performer<'a> {
    pub grid: &'a mut Grid,
    pub cursor: &'a mut Cursor,
}

impl<'a> vte::Perform for Performer<'a> {
    fn print(&mut self, c: char) {
        if self.cursor.col < self.grid.cols && self.cursor.row < self.grid.rows {
            let mut cell = self.cursor.cell; // Cell is Copy
            cell.ch = c;
            self.grid.cells[self.cursor.row * self.grid.cols + self.cursor.col] = cell;
        }
        self.cursor.col += 1;
    }
    fn execute(&mut self, byte: u8) {
        match byte {
            b'\n' => self.cursor.row += 1,
            b'\r' => self.cursor.col = 0,
            b'\x08' => self.cursor.col = self.cursor.col.saturating_sub(1),
            _ => (),
        }
    }

    fn csi_dispatch(
        &mut self,
        params: &vte::Params,
        _intermediates: &[u8],
        _ignore: bool,
        action: char,
    ) {
        if action == 'm' {
            for param in params.iter() {
                let code = param[0];
                match code {
                    0 => {
                        self.cursor.cell.fg = [0xe0, 0xe0, 0xe0];
                    }
                    30..=37 => self.cursor.cell.fg = ansi_color(code - 30),
                    40..=47 => self.cursor.cell.bg = ansi_color(code - 40),
                    _ => {}
                }
            }
        }
    }
}

fn ansi_color(code: u16) -> [u8; 3] {
    match code {
        0 => [0, 0, 0],
        1 => [205, 49, 49],
        2 => [13, 188, 121],
        _ => [255, 255, 255],
    }
}

pub fn feed(parser: &mut vte::Parser, grid: &mut Grid, cursor: &mut Cursor, bytes: &[u8]) {
    // construct performer
    let mut performer = Performer { grid, cursor };

    parser.advance(&mut performer, bytes);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prints_hello_world() {
        let mut grid = Grid::new(80, 24);
        let mut cursor = Cursor {
            col: 0,
            row: 0,
            cell: Cell::default(),
        };
        let mut parser = vte::Parser::new();
        feed(&mut parser, &mut grid, &mut cursor, b"hello");
        assert_eq!(grid.cells[0].ch, 'h');
        assert_eq!(grid.cells[1].ch, 'e');
        assert_eq!(grid.cells[2].ch, 'l');
        assert_eq!(grid.cells[3].ch, 'l');
    }

    #[test]
    fn backspace() {
        let mut grid = Grid::new(80, 24);
        let mut cursor = Cursor {
            col: 0,
            row: 0,
            cell: Cell::default(),
        };
        let mut parser = vte::Parser::new();
        feed(&mut parser, &mut grid, &mut cursor, b"hello\x08");
        assert_eq!(cursor.col, 4);
    }

    #[test]
    fn newline() {
        let mut grid = Grid::new(80, 24);
        let mut cursor = Cursor {
            col: 0,
            row: 0,
            cell: Cell::default(),
        };
        let mut parser = vte::Parser::new();
        feed(&mut parser, &mut grid, &mut cursor, b"hello\r\n");
        assert_eq!(cursor.row, 1);
        assert_eq!(cursor.col, 0);
    }
}
