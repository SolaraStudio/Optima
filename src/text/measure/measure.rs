pub struct TextMeasurer;

impl TextMeasurer {
    pub fn measure_text(text: &str, font_size: f32) -> f32 {
        text.chars().map(|c| Self::char_width(c, font_size)).sum()
    }

    pub fn measure_text_with_font(text: &str, font_size: f32, font_name: &str) -> f32 {
        let glyphs = super::shaping::TextShaper::shape(text, font_size, font_name);
        glyphs.iter().map(|g| g.x_advance as f32).sum()
    }

    pub fn measure_line_height(font_size: f32) -> f32 {
        font_size * 1.2
    }

    pub fn measure_paragraph(text: &str, font_size: f32, max_width: f32) -> (usize, f32) {
        let mut lines = 1;
        let mut current_width = 0.0;
        for c in text.chars() {
            let width = Self::char_width(c, font_size);
            if current_width + width > max_width {
                lines += 1;
                current_width = width;
            } else {
                current_width += width;
            }
        }
        (lines, lines as f32 * Self::measure_line_height(font_size))
    }

    fn char_width(c: char, font_size: f32) -> f32 {
        match c {
            ' ' => font_size * 0.3,
            'I' | 'i' | 'l' | '1' | '|' => font_size * 0.3,
            'M' | 'W' | 'w' => font_size * 0.8,
            _ => font_size * 0.5,
        }
    }
}
