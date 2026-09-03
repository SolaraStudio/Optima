use crate::css::computed::ComputedStyle;

pub struct ResolveContext {
    pub viewport_width: f32,
    pub viewport_height: f32,
    pub root_font_size: f32,
}

impl Default for ResolveContext {
    fn default() -> Self {
        ResolveContext {
            viewport_width: 800.0,
            viewport_height: 600.0,
            root_font_size: 16.0,
        }
    }
}

impl ResolveContext {
    pub fn resolve_length(&self, style: &ComputedStyle, property: &str) -> f32 {
        if let Some(val) = style.get(property) {
            match val {
                crate::css::value::Value::Length(l) => l.to_px(self.root_font_size),
                crate::css::value::Value::Percentage(p) => self.viewport_width * p / 100.0,
                crate::css::value::Value::Keyword(k) if k == "auto" => 0.0,
                _ => 0.0,
            }
        } else {
            0.0
        }
    }

    pub fn resolve_width(&self, style: &ComputedStyle) -> f32 {
        let mut w = self.resolve_length(style, "width");
        if w == 0.0 {
            w = self.viewport_width;
        }
        w
    }

    pub fn resolve_height(&self, style: &ComputedStyle) -> f32 {
        self.resolve_length(style, "height")
    }
}
