use super::glyph::Glyph;

#[derive(Debug, Clone)]
pub struct Line {
    pub glyphs: Vec<Glyph>,
    pub width: f32,
    pub height: f32,
    pub ascent: f32,
    pub descent: f32,
    pub leading: f32,
}

impl Line {
    pub fn new() -> Self {
        Self {
            glyphs: Vec::new(),
            width: 0.0,
            height: 0.0,
            ascent: 0.0,
            descent: 0.0,
            leading: 0.0,
        }
    }

    pub fn add_glyph(&mut self, glyph: Glyph) {
        self.glyphs.push(glyph);
        self.width += glyph.x_advance;
        self.height = self.height.max(glyph.y_offset.abs() + glyph.y_advance.abs());
    }

    pub fn add_glyphs(&mut self, glyphs: &[Glyph]) {
        for glyph in glyphs {
            self.add_glyph(*glyph);
        }
    }

    pub fn get_glyphs(&self) -> &[Glyph] {
        &self.glyphs
    }

    pub fn get_width(&self) -> f32 {
        self.width
    }

    pub fn get_height(&self) -> f32 {
        self.height
    }

    pub fn is_empty(&self) -> bool {
        self.glyphs.is_empty()
    }

    pub fn clear(&mut self) {
        self.glyphs.clear();
        self.width = 0.0;
        self.height = 0.0;
        self.ascent = 0.0;
        self.descent = 0.0;
        self.leading = 0.0;
    }

    pub fn extend(&mut self, other: &Line) {
        self.glyphs.extend(other.glyphs.clone());
        self.width += other.width;
        self.height = self.height.max(other.height);
    }

    pub fn split_at(&self, index: usize) -> (Line, Line) {
        let mut left = Line::new();
        let mut right = Line::new();
        for (i, glyph) in self.glyphs.iter().enumerate() {
            if i < index {
                left.add_glyph(*glyph);
            } else {
                right.add_glyph(*glyph);
            }
        }
        (left, right)
    }
}

impl Default for Line {
    fn default() -> Self {
        Self::new()
    }
}
