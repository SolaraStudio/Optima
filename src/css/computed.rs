use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct ComputedStyle {
    pub color: Option<String>,
    pub background: Option<String>,
    pub font_size: Option<f32>,
    pub width: Option<f32>,
    pub height: Option<f32>,
    pub display: Option<String>,
    pub margin: Option<f32>,
    pub padding: Option<f32>,
    pub border: Option<f32>,
}

impl ComputedStyle {
    pub fn new() -> Self {
        Self {
            color: None,
            background: None,
            font_size: None,
            width: None,
            height: None,
            display: None,
            margin: None,
            padding: None,
            border: None,
        }
    }

    pub fn from_declarations(decls: &HashMap<String, String>) -> Self {
        let mut style = Self::new();
        for (key, value) in decls {
            match key.as_str() {
                "color" => style.color = Some(value.clone()),
                "background" => style.background = Some(value.clone()),
                "font-size" => {
                    if let Ok(v) = value.trim_end_matches("px").parse::<f32>() {
                        style.font_size = Some(v);
                    }
                }
                "width" => {
                    if let Ok(v) = value.trim_end_matches("px").parse::<f32>() {
                        style.width = Some(v);
                    }
                }
                "height" => {
                    if let Ok(v) = value.trim_end_matches("px").parse::<f32>() {
                        style.height = Some(v);
                    }
                }
                "display" => style.display = Some(value.clone()),
                "margin" => {
                    if let Ok(v) = value.trim_end_matches("px").parse::<f32>() {
                        style.margin = Some(v);
                    }
                }
                "padding" => {
                    if let Ok(v) = value.trim_end_matches("px").parse::<f32>() {
                        style.padding = Some(v);
                    }
                }
                "border" => {
                    if let Ok(v) = value.trim_end_matches("px").parse::<f32>() {
                        style.border = Some(v);
                    }
                }
                _ => {}
            }
        }
        style
    }

    pub fn inherit_from(&mut self, parent: &ComputedStyle) {
        if self.color.is_none() {
            self.color = parent.color.clone();
        }
        if self.font_size.is_none() {
            self.font_size = parent.font_size;
        }
        if self.display.is_none() {
            self.display = parent.display.clone();
        }
    }
}
