use crate::grid::{Cell, Grid};

// add <Cell> in later for coloring
pub struct Cursor {
    pub col: usize,
    pub row: usize,
}

pub struct Performer<'a> {
    pub grid: &'a mut Grid,
    pub cursor: &'a mut Cursor,
}

impl<'a> vte::Perform for Performer<'a> {
    fn print(&mut self, c: char) {
        self.grid.put(self.cursor.col, self.cursor.row, c);
        self.cursor.col += 1;
    }
}

pub fn feed(parser: &mut vte::Parser, grid: &mut Grid, cursor: &mut Cursor, bytes: &[u8]) {
    // construct performer
    let mut performer = Performer { grid, cursor };

    parser.advance(&mut performer, bytes);
}

#[test]
fn prints_hello_world() {
    let mut grid = Grid::new(80, 24);
    let mut cursor = Cursor { col: 0, row: 0 };
    let mut parser = vte::Parser::new();
    feed(&mut parser, &mut grid, &mut cursor, b"hello");
    assert_eq!(grid.cells[0].ch, 'h');
    assert_eq!(grid.cells[1].ch, 'e');
    assert_eq!(grid.cells[2].ch, 'l');
    assert_eq!(grid.cells[3].ch, 'l');
}
