use crate::css::computed::ComputedStyle;

pub struct MeasureContext {
    pub font_size: f32,
    pub font_family: String,
}

impl MeasureContext {
    pub fn from_style(style: &ComputedStyle) -> Self {
        let font_size = style
            .get("font-size")
            .and_then(|v| v.as_length())
            .map(|l| l.to_px(16.0))
            .unwrap_or(16.0);
        let font_family = style
            .get("font-family")
            .and_then(|v| v.as_string())
            .unwrap_or("sans-serif")
            .to_string();
        MeasureContext {
            font_size,
            font_family,
        }
    }

    pub fn measure_text(&self, text: &str) -> (f32, f32) {
        let width = text.len() as f32 * self.font_size * 0.6;
        let height = self.font_size * 1.2;
        (width, height)
    }
}
