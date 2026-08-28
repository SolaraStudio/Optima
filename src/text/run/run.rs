use super::glyph::Glyph;
use super::font::FontLoader;

#[derive(Debug, Clone)]
pub struct Run {
    pub text: String,
    pub font_name: String,
    pub font_size: f32,
    pub glyphs: Vec<Glyph>,
    pub width: f32,
    pub height: f32,
}

impl Run {
    pub fn new(text: &str, font_name: &str, font_size: f32) -> Self {
        Self {
            text: text.to_string(),
            font_name: font_name.to_string(),
            font_size,
            glyphs: Vec::new(),
            width: 0.0,
            height: 0.0,
        }
    }

    pub fn shape(&mut self) {
        let glyphs = super::shaping::TextShaper::shape(&self.text, self.font_size, &self.font_name);
        for info in glyphs {
            let glyph = Glyph::from_info(info, self.font_size);
            self.width += glyph.x_advance;
            self.height = self.height.max(glyph.y_offset.abs() + glyph.y_advance.abs());
            self.glyphs.push(glyph);
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

    pub fn get_text(&self) -> &str {
        &self.text
    }

    pub fn get_font_name(&self) -> &str {
        &self.font_name
    }

    pub fn get_font_size(&self) -> f32 {
        self.font_size
    }

    pub fn is_empty(&self) -> bool {
        self.text.is_empty()
    }
}
