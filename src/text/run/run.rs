use crate::text::glyph::Glyph;

#[derive(Debug, Clone)]
pub struct TextRun {
    pub text: String,
    pub glyphs: Vec<Glyph>,
    pub start_offset: f32,
    pub font_size: f32,
    pub font_family: String,
}

impl TextRun {
    pub fn new(text: &str, font_size: f32, font_family: &str) -> Self {
        TextRun {
            text: text.to_string(), glyphs: Vec::new(), start_offset: 0.0,
            font_size, font_family: font_family.to_string(),
        }
    }

    pub fn set_glyphs(&mut self, glyphs: Vec<Glyph>) { self.glyphs = glyphs; }

    pub fn advance_width(&self) -> f32 {
        self.glyphs.iter().map(|g| g.advance_x).sum()
    }
}
