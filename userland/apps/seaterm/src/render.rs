use crate::font::FontCache;
use crate::grid::Grid;

/// Buffer layout: little-endian ARGB8888, so bytes are [B, G, R, A] per pixel,
pub fn paint(
    buf: &mut [u8],
    buf_w: usize,
    buf_h: usize,
    stride: usize,
    grid: &Grid,
    font: &mut FontCache,
) {
    let bg = grid.cells.first().map(|c| c.bg).unwrap_or([0x1e; 3]);
    for y in 0..buf_h {
        let row = &mut buf[y * stride..y * stride + buf_w * 4];
        for px in row.chunks_exact_mut(4) {
            px[0] = bg[2]; // B
            px[1] = bg[1]; // G
            px[2] = bg[0]; // R
            px[3] = 0xff; // A
        }
    }

    let cell_w = font.cell_w;
    let cell_h = font.cell_h;
    let ascent = font.ascent;

    for row in 0..grid.rows {
        for col in 0..grid.cols {
            let cell = grid.cells[row * grid.cols + col];
            if cell.ch == ' ' {
                continue; // background already filled
            }

            let glyph = font.glyph(cell.ch);

            // Glyph origin within the cell.
            let cell_x = (col * cell_w) as i32;
            let cell_y = (row * cell_h) as i32;

            let glyph_x = cell_x + glyph.xmin;
            let glyph_y = cell_y + ascent - glyph.ymin - glyph.height as i32;

            let fg = cell.fg;

            for gy in 0..glyph.height {
                let py = glyph_y + gy as i32;
                if py < 0 || py >= buf_h as i32 {
                    continue;
                }
                for gx in 0..glyph.width {
                    let px = glyph_x + gx as i32;
                    if px < 0 || px >= buf_w as i32 {
                        continue;
                    }
                    let alpha = glyph.bitmap[gy * glyph.width + gx];
                    if alpha == 0 {
                        continue;
                    }
                    let off = py as usize * stride + px as usize * 4;
                    // Source-over blend: out = fg*a + bg*(1-a)
                    let a = alpha as u32;
                    let inv = 255 - a;
                    let b = (fg[2] as u32 * a + buf[off] as u32 * inv) / 255;
                    let g = (fg[1] as u32 * a + buf[off + 1] as u32 * inv) / 255;
                    let r = (fg[0] as u32 * a + buf[off + 2] as u32 * inv) / 255;
                    buf[off] = b as u8;
                    buf[off + 1] = g as u8;
                    buf[off + 2] = r as u8;
                    buf[off + 3] = 0xff;
                }
            }
        }
    }
}
