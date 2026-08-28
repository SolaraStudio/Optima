use super::shaping::TextShaper;

pub struct TextRenderer;

impl TextRenderer {
    pub fn render(text: &str, x: f32, y: f32, size: f32, color: (u8, u8, u8)) -> Vec<(f32, f32, u32)> {
        let glyphs = TextShaper::shape_with_fallback(text, size);
        let mut positions = Vec::new();
        let mut current_x = x;
        let (r, g, b) = color;
        let packed_color = ((r as u32) << 16) | ((g as u32) << 8) | (b as u32);
        for glyph in glyphs {
            let pos_x = current_x + glyph.x_offset as f32;
            let pos_y = y + glyph.y_offset as f32;
            positions.push((pos_x, pos_y, packed_color));
            current_x += glyph.x_advance as f32;
        }
        positions
    }

    pub fn measure_text(text: &str, size: f32) -> f32 {
        let glyphs = TextShaper::shape_with_fallback(text, size);
        glyphs.iter().map(|g| g.x_advance as f32).sum()
    }
}
