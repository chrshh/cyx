use fontdue::{Font, FontSettings, Metrics};
use std::collections::HashMap;

static FONT_BYTES: &[u8] = include_bytes!("../assets/JetBrainsMonoNerdFontMono-Regular.ttf");

/* rasterized glyph */
pub struct Glyph {
    pub width: usize,
    pub height: usize,
    pub xmin: i32,
    pub ymin: i32,
    pub bitmap: Vec<u8>,
}

pub struct FontCache {
    font: Font,
    px_size: f32,
    cache: HashMap<char, Glyph>,

    pub cell_w: usize,
    pub cell_h: usize,
    pub ascent: i32,
}

impl FontCache {
    pub fn new(px_size: f32) -> Self {
        let font = Font::from_bytes(FONT_BYTES, FontSettings::default())
            .expect("failed to parse embedded font");

        let (m_metrics, _) = font.rasterize('M', px_size);
        let cell_w = m_metrics.advance_width.ceil() as usize;

        let line_metrics = font
            .horizontal_line_metrics(px_size)
            .expect("font missing line metrics");
        let ascent = line_metrics.ascent.ceil() as i32;
        let descent = (-line_metrics.descent).ceil() as i32;
        let linegap = line_metrics.line_gap.ceil() as i32;
        let cell_h = (ascent + descent + linegap) as usize;

        Self {
            font,
            px_size,
            cache: HashMap::new(),
            cell_w,
            cell_h,
            ascent,
        }
    }

    pub fn glyph(&mut self, ch: char) -> &Glyph {
        self.cache.entry(ch).or_insert_with(|| {
            let (metrics, bitmap) = self.font.rasterize(ch, self.px_size);
            Glyph {
                width: metrics.width,
                height: metrics.height,
                xmin: metrics.xmin,
                ymin: metrics.ymin,
                bitmap,
            }
        })
    }
}

#[allow(dead_code)]
fn _force_use_metrics(_: Metrics) {}
