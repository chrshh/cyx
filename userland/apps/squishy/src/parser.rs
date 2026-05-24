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

impl<'a> Performer<'a> {
    fn newline(&mut self) {
        self.cursor.row += 1;
        if self.cursor.row >= self.grid.rows {
            self.grid.scroll_up();
            self.cursor.row = self.grid.rows - 1;
        }
    }
}

impl<'a> vte::Perform for Performer<'a> {
    fn print(&mut self, c: char) {
        if self.cursor.col >= self.grid.cols {
            self.cursor.col = 0;
            self.newline();
        }

        let mut cell = self.cursor.cell;
        cell.ch = c;
        let idx = self.cursor.row * self.grid.cols + self.cursor.col;
        self.grid.cells[idx] = cell;
        self.cursor.col += 1;
    }

    fn execute(&mut self, byte: u8) {
        match byte {
            b'\n' => self.newline(),
            b'\r' => self.cursor.col = 0,
            b'\x08' => self.cursor.col = self.cursor.col.saturating_sub(1),
            _ => (),
        }
    }

    fn csi_dispatch(
        &mut self,
        params: &vte::Params,
        intermediates: &[u8],
        ignore: bool,
        action: char,
    ) {
        if ignore {
            return;
        }

        if intermediates == b"?" {
            let code = params.iter().next().map(|p| p[0]).unwrap_or(0);
            match (code, action) {
                (1049, 'h') => {
                    self.grid.enter_alt(self.cursor.col, self.cursor.row);
                    self.cursor.col = 0;
                    self.cursor.row = 0;
                    self.cursor.cell = Cell::default();
                }
                (1049, 'l') => {
                    if let Some((col, row)) = self.grid.exit_alt() {
                        self.cursor.col = col;
                        self.cursor.row = row;
                        self.cursor.cell = Cell::default();
                    }
                }
                _ => {}
            }
            return;
        }

        match action {
            /* colors */
            'm' => {
                if params.is_empty() {
                    self.cursor.cell = Cell::default();
                    return;
                }

                for param in params.iter() {
                    let code = param[0];
                    match code {
                        0 => {
                            self.cursor.cell = Cell::default();
                        }
                        30..=37 => self.cursor.cell.fg = ansi_color(code - 30),
                        39 => self.cursor.cell.default_fg(),
                        40..=47 => self.cursor.cell.bg = ansi_color(code - 40),
                        49 => self.cursor.cell.default_bg(),
                        _ => {}
                    }
                }
            }
            /* cursor movements */
            'A' => {
                // UP
                let n = params.iter().next().map(|p| p[0]).unwrap_or(1).max(1) as usize;
                self.cursor.row = self.cursor.row.saturating_sub(n);
            }
            'B' => {
                // DOWN
                let n = params.iter().next().map(|p| p[0]).unwrap_or(1).max(1) as usize;
                self.cursor.row = (self.cursor.row + n).min(self.grid.rows.saturating_sub(1));
            }
            'C' => {
                // RIGHT
                let n = params.iter().next().map(|p| p[0]).unwrap_or(1).max(1) as usize;
                self.cursor.col = (self.cursor.col + n).min(self.grid.cols.saturating_sub(1));
            }
            'D' => {
                // LEFT
                let n = params.iter().next().map(|p| p[0]).unwrap_or(1).max(1) as usize;
                self.cursor.col = self.cursor.col.saturating_sub(n);
            }
            'H' => {
                // cursor reset
                let mut it = params.iter();
                let row = it.next().map(|p| p[0]).unwrap_or(1).max(1) as usize - 1;
                let col = it.next().map(|p| p[0]).unwrap_or(1).max(1) as usize - 1;
                self.cursor.row = row.min(self.grid.rows.saturating_sub(1));
                self.cursor.col = col.min(self.grid.cols.saturating_sub(1));
            }

            /* erase line / display */
            'J' => {
                let n = params.iter().next().map(|p| p[0]).unwrap_or(0) as usize;
                let pos = self.cursor.row * self.grid.cols + self.cursor.col;
                let end = self.grid.rows * self.grid.cols;
                match n {
                    0 => self.grid.clear_range(pos, end), // cursor -> end of screen
                    1 => self.grid.clear_range(0, pos + 1), // start of screen -> cursor
                    2 => self.grid.clear_range(0, end),   // full screen
                    _ => {}
                }
            }
            'K' => {
                let n = params.iter().next().map(|p| p[0]).unwrap_or(0) as usize;
                let row_start = self.cursor.row * self.grid.cols;
                let row_end = row_start + self.grid.cols;
                let pos = row_start + self.cursor.col;
                match n {
                    0 => self.grid.clear_range(pos, row_end), // cursor -> end of line
                    1 => self.grid.clear_range(row_start, pos + 1), // start of line -> cursor
                    2 => self.grid.clear_range(row_start, row_end), // whole line
                    _ => {}
                }
            }

            _ => (),
        }
    }

    /* def behavior: void */
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
    let mut performer = Performer { grid, cursor };
    parser.advance(&mut performer, bytes);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup() -> (vte::Parser, Grid, Cursor) {
        (
            vte::Parser::new(),
            Grid::new(10, 5),
            Cursor {
                col: 0,
                row: 0,
                cell: Cell::default(),
            },
        )
    }

    #[test]
    fn prints_hello() {
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

    #[test]
    fn cursor_up() {
        let mut grid = Grid::new(80, 24);
        let mut cursor = Cursor {
            col: 0,
            row: 5,
            cell: Cell::default(),
        };
        let mut parser = vte::Parser::new();
        feed(&mut parser, &mut grid, &mut cursor, b"\x1b[3A");
        assert_eq!(cursor.row, 2);
    }

    #[test]
    fn cursor_down() {
        let mut grid = Grid::new(80, 24);
        let mut cursor = Cursor {
            col: 0,
            row: 23,
            cell: Cell::default(),
        };
        let mut parser = vte::Parser::new();
        feed(&mut parser, &mut grid, &mut cursor, b"\x1b[3B");
        assert_eq!(cursor.row, 23);
    }

    #[test]
    fn cursor_left() {
        let mut grid = Grid::new(80, 24);
        let mut cursor = Cursor {
            col: 0,
            row: 23,
            cell: Cell::default(),
        };
        let mut parser = vte::Parser::new();
        feed(&mut parser, &mut grid, &mut cursor, b"\x1b[3D");
        assert_eq!(cursor.col, 0);
    }

    #[test]
    fn cursor_right() {
        let mut grid = Grid::new(80, 24);
        let mut cursor = Cursor {
            col: 79,
            row: 23,
            cell: Cell::default(),
        };
        let mut parser = vte::Parser::new();
        feed(&mut parser, &mut grid, &mut cursor, b"\x1b[3C");
        assert_eq!(cursor.col, 79);
    }

    #[test]
    fn cursor_position() {
        let mut grid = Grid::new(80, 24);
        let mut cursor = Cursor {
            col: 0,
            row: 0,
            cell: Cell::default(),
        };
        let mut parser = vte::Parser::new();
        feed(&mut parser, &mut grid, &mut cursor, b"\x1b[10;20H");
        assert_eq!(cursor.row, 9);
        assert_eq!(cursor.col, 19);
    }

    #[test]
    fn erase_to_end_of_line() {
        let mut grid = Grid::new(10, 1);
        let mut cursor = Cursor {
            col: 0,
            row: 0,
            cell: Cell::default(),
        };
        let mut parser = vte::Parser::new();
        feed(
            &mut parser,
            &mut grid,
            &mut cursor,
            b"abcdefghij\x1b[1;6H\x1b[K",
        );
        assert_eq!(grid.cells[4].ch, 'e');
        assert_eq!(grid.cells[5].ch, ' ');
        assert_eq!(grid.cells[9].ch, ' ');
        assert_eq!(cursor.col, 5);
    }

    /* -- cursor motions -- */

    #[test]
    fn cursor_up_default() {
        let (mut p, mut g, mut c) = setup();
        c.row = 3;
        feed(&mut p, &mut g, &mut c, b"\x1b[A");
        assert_eq!(c.row, 2);
    }

    #[test]
    fn cursor_up_saturates_at_zero() {
        let (mut p, mut g, mut c) = setup();
        c.row = 1;
        feed(&mut p, &mut g, &mut c, b"\x1b[5A");
        assert_eq!(c.row, 0);
    }

    #[test]
    fn cursor_down_n() {
        let (mut p, mut g, mut c) = setup();
        feed(&mut p, &mut g, &mut c, b"\x1b[2B");
        assert_eq!(c.row, 2);
    }

    #[test]
    fn cursor_right_n() {
        let (mut p, mut g, mut c) = setup();
        feed(&mut p, &mut g, &mut c, b"\x1b[4C");
        assert_eq!(c.col, 4);
    }

    #[test]
    fn cursor_left_n() {
        let (mut p, mut g, mut c) = setup();
        c.col = 5;
        feed(&mut p, &mut g, &mut c, b"\x1b[3D");
        assert_eq!(c.col, 2);
    }

    #[test]
    fn cursor_left_saturates_at_zero() {
        let (mut p, mut g, mut c) = setup();
        c.col = 2;
        feed(&mut p, &mut g, &mut c, b"\x1b[9D");
        assert_eq!(c.col, 0);
    }

    #[test]
    fn cursor_position_default_is_top_left() {
        let (mut p, mut g, mut c) = setup();
        c.row = 3;
        c.col = 4;
        feed(&mut p, &mut g, &mut c, b"\x1b[H");
        assert_eq!(c.row, 0);
        assert_eq!(c.col, 0);
    }

    #[test]
    fn cursor_position_only_row() {
        let (mut p, mut g, mut c) = setup();
        feed(&mut p, &mut g, &mut c, b"\x1b[4H");
        assert_eq!(c.row, 3);
        assert_eq!(c.col, 0);
    }

    /* -- Erase display & line -- */

    #[test]
    fn erase_display_to_end() {
        let (mut p, mut g, mut c) = setup();
        for cell in &mut g.cells {
            cell.ch = 'x';
        }
        c.row = 2;
        c.col = 3;
        feed(&mut p, &mut g, &mut c, b"\x1b[J");
        assert_eq!(g.cells[0].ch, 'x');
        assert_eq!(g.cells[2 * 10 + 2].ch, 'x');
        assert_eq!(g.cells[2 * 10 + 3].ch, ' ');
        assert_eq!(g.cells[g.cells.len() - 1].ch, ' ');
        assert_eq!(c.row, 2);
        assert_eq!(c.col, 3);
    }

    #[test]
    fn erase_display_to_start() {
        let (mut p, mut g, mut c) = setup();
        for cell in &mut g.cells {
            cell.ch = 'x';
        }
        c.row = 2;
        c.col = 3;
        feed(&mut p, &mut g, &mut c, b"\x1b[1J");
        assert_eq!(g.cells[0].ch, ' ');
        assert_eq!(g.cells[2 * 10 + 3].ch, ' ');
        assert_eq!(g.cells[2 * 10 + 4].ch, 'x');
        assert_eq!(g.cells[g.cells.len() - 1].ch, 'x');
    }

    #[test]
    fn erase_display_all() {
        let (mut p, mut g, mut c) = setup();
        for cell in &mut g.cells {
            cell.ch = 'x';
        }
        feed(&mut p, &mut g, &mut c, b"\x1b[2J");
        for cell in &g.cells {
            assert_eq!(cell.ch, ' ');
        }
    }

    #[test]
    fn erase_line_to_start() {
        let (mut p, mut g, mut c) = setup();
        feed(&mut p, &mut g, &mut c, b"abcdefghij");
        feed(&mut p, &mut g, &mut c, b"\x1b[1;6H\x1b[1K");
        assert_eq!(g.cells[0].ch, ' ');
        assert_eq!(g.cells[5].ch, ' ');
        assert_eq!(g.cells[6].ch, 'g');
        assert_eq!(g.cells[9].ch, 'j');
    }

    #[test]
    fn erase_line_all() {
        let (mut p, mut g, mut c) = setup();
        feed(&mut p, &mut g, &mut c, b"abcdefghij");
        feed(&mut p, &mut g, &mut c, b"\x1b[2;1Hxxxxx");
        feed(&mut p, &mut g, &mut c, b"\x1b[1;5H\x1b[2K");
        for col in 0..10 {
            assert_eq!(g.cells[col].ch, ' ');
        }
        assert_eq!(g.cells[10].ch, 'x');
        assert_eq!(g.cells[14].ch, 'x');
    }

    #[test]
    fn cup_then_print_writes_at_target() {
        let (mut p, mut g, mut c) = setup();
        feed(&mut p, &mut g, &mut c, b"\x1b[3;5HX");
        assert_eq!(g.cells[2 * 10 + 4].ch, 'X');
        assert_eq!(c.row, 2);
        assert_eq!(c.col, 5);
    }

    /* -- scrolling / wrapping -- */

    #[test]
    fn print_wraps_at_right_edge() {
        let (mut p, mut g, mut c) = setup(); // 10x5 grid
        feed(&mut p, &mut g, &mut c, b"abcdefghijkl"); // 12 chars on a 10-wide grid
        assert_eq!(g.cells[9].ch, 'j'); // last of first row
        assert_eq!(g.cells[10].ch, 'k'); // first of second row
        assert_eq!(g.cells[11].ch, 'l');
        assert_eq!(c.row, 1);
        assert_eq!(c.col, 2);
    }

    #[test]
    fn print_scrolls_off_bottom() {
        let (mut p, mut g, mut c) = setup(); // 10x5 grid
        // Print 6 rows of distinct chars.
        feed(
            &mut p,
            &mut g,
            &mut c,
            b"aaaaaaaaaa\r\nbbbbbbbbbb\r\ncccccccccc\r\ndddddddddd\r\neeeeeeeeee\r\nffffffffff",
        );
        // Top row should now be 'b' (was second row before scroll).
        assert_eq!(g.cells[0].ch, 'b');
        // Bottom row is 'f'.
        assert_eq!(g.cells[40].ch, 'f');
    }
}
