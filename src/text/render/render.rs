use crate::text::glyph::Glyph;
use crate::css::colors::Color;

pub struct TextRenderer {
    pub font_size: f32,
    pub color: Color,
    pub antialias: bool,
}

impl TextRenderer {
    pub fn new(font_size: f32, color: Color) -> Self {
        TextRenderer { font_size, color, antialias: true }
    }

    pub fn render_glyph(&self, glyph: &Glyph, x: f32, y: f32) -> TextRenderCommand {
        TextRenderCommand { glyph_id: glyph.id, x, y, width: glyph.width, height: glyph.height, color: self.color }
    }
}

#[derive(Debug, Clone)]
pub struct TextRenderCommand {
    pub glyph_id: u32,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub color: Color,
}
