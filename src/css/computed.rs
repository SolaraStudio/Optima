use std::collections::HashMap;

pub struct ComputedStyle {
    pub color: Option<String>,
    pub background: Option<String>,
    pub font_size: Option<f32>,
    pub width: Option<f32>,
    pub height: Option<f32>,
}

impl ComputedStyle {
    pub fn new() -> Self {
        Self {
            color: None,
            background: None,
            font_size: None,
            width: None,
            height: None,
        }
    }

    pub fn merge(&mut self, other: ComputedStyle) {
        if let Some(v) = other.color { self.color = Some(v); }
        if let Some(v) = other.background { self.background = Some(v); }
        if let Some(v) = other.font_size { self.font_size = Some(v); }
        if let Some(v) = other.width { self.width = Some(v); }
        if let Some(v) = other.height { self.height = Some(v); }
    }
}
