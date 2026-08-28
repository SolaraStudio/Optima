use super::shaping::TextShaper;
use super::font::FontLoader;
use super::glyph::Glyph;
use super::line::Line;
use super::run::Run;

pub struct TextLayout {
    pub lines: Vec<Line>,
    pub width: f32,
    pub height: f32,
}

impl TextLayout {
    pub fn new() -> Self {
        Self {
            lines: Vec::new(),
            width: 0.0,
            height: 0.0,
        }
    }

    pub fn layout(&mut self, text: &str, font_size: f32, max_width: f32, font_name: &str) {
        let glyphs = TextShaper::shape(text, font_size, font_name);
        let mut current_line = Line::new();
        let mut current_width = 0.0;
        let mut current_height = font_size;

        for glyph_info in glyphs {
            let glyph = Glyph::from_info(glyph_info, font_size);
            let advance = glyph.x_advance;
            if current_width + advance > max_width && !current_line.is_empty() {
                self.lines.push(current_line);
                current_line = Line::new();
                current_width = 0.0;
                current_height += font_size * 1.2;
            }
            current_line.add_glyph(glyph);
            current_width += advance;
        }

        if !current_line.is_empty() {
            self.lines.push(current_line);
        }

        self.width = max_width;
        self.height = current_height + font_size;
    }

    pub fn layout_with_fallback(&mut self, text: &str, font_size: f32, max_width: f32) {
        self.layout(text, font_size, max_width, "sans-serif");
    }

    pub fn get_line_count(&self) -> usize {
        self.lines.len()
    }

    pub fn get_glyph_count(&self) -> usize {
        self.lines.iter().map(|l| l.glyphs.len()).sum()
    }

    pub fn get_width(&self) -> f32 {
        self.width
    }

    pub fn get_height(&self) -> f32 {
        self.height
    }

    pub fn clear(&mut self) {
        self.lines.clear();
        self.width = 0.0;
        self.height = 0.0;
    }
}
