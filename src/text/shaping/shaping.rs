use crate::text::font::FontFace;
use crate::text::glyph::{Glyph, GlyphRun};

pub struct TextShaper;

impl TextShaper {
    pub fn shape(text: &str, font_size: f32) -> GlyphRun {
        let mut run = GlyphRun::new();
        run.font_size = font_size;
        let _em_width = font_size;
        for ch in text.chars() {
            if ch == '\n' || ch == '\r' || ch == '\t' {
                continue;
            }
            let glyph_id = ch as u32;
            let advance = font_size * 0.6;
            let mut glyph = Glyph::new(glyph_id);
            glyph.advance_x = advance;
            glyph.width = advance;
            glyph.height = font_size;
            run.add_glyph(glyph);
        }
        run
    }

    pub fn shape_with_font(text: &str, _font: &FontFace, font_size: f32) -> GlyphRun {
        Self::shape(text, font_size)
    }
}
