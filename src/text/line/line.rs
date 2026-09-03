use crate::text::glyph::Glyph;

#[derive(Debug, Clone, Default)]
pub struct TextLine {
    pub y: f32,
    pub height: f32,
    pub max_width: f32,
    pub glyphs: Vec<(Glyph, f32)>,
    pub baseline: f32,
}

impl TextLine {
    pub fn new(_x: f32, y: f32, max_width: f32) -> Self {
        TextLine { y, height: 0.0, max_width, glyphs: Vec::new(), baseline: 0.0 }
    }

    pub fn add_glyph(&mut self, glyph: Glyph, x: f32) {
        self.height = self.height.max(glyph.height);
        self.glyphs.push((glyph, x));
    }

    pub fn width(&self) -> f32 {
        self.glyphs.last().map(|(g, x)| x + g.advance_x).unwrap_or(0.0)
    }

    pub fn has_glyphs(&self) -> bool { !self.glyphs.is_empty() }

    pub fn hit_test(&self, x: f32) -> Option<usize> {
        for (glyph, gx) in &self.glyphs {
            if x >= *gx && x <= *gx + glyph.advance_x {
                return Some(*gx as usize);
            }
        }
        None
    }
}
