use crate::text::font::FontMetrics;

pub struct TextMeasurer {
    pub font_size: f32,
    pub metrics: FontMetrics,
}

impl TextMeasurer {
    pub fn new(font_size: f32, metrics: FontMetrics) -> Self {
        TextMeasurer { font_size, metrics }
    }

    pub fn measure(&self, text: &str) -> (f32, f32) {
        let width = text.len() as f32 * self.font_size * 0.6;
        let height = self.font_size * 1.2;
        (width, height)
    }

    pub fn measure_char(&self, _ch: char) -> (f32, f32) {
        (self.font_size * 0.6, self.font_size)
    }

    pub fn line_height(&self) -> f32 { self.font_size * 1.2 }

    pub fn baseline(&self) -> f32 { self.font_size * 0.8 }
}
