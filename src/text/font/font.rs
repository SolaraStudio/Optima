use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FontWeight {
    Thin = 100,
    ExtraLight = 200,
    Light = 300,
    Regular = 400,
    Medium = 500,
    SemiBold = 600,
    Bold = 700,
    ExtraBold = 800,
    Black = 900,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FontStyle {
    Normal,
    Italic,
    Oblique,
}

#[derive(Debug, Clone)]
pub struct FontMetrics {
    pub ascent: f32,
    pub descent: f32,
    pub line_gap: f32,
    pub cap_height: f32,
    pub x_height: f32,
    pub avg_char_width: f32,
}

impl Default for FontMetrics {
    fn default() -> Self {
        FontMetrics {
            ascent: 12.0,
            descent: -4.0,
            line_gap: 2.0,
            cap_height: 8.0,
            x_height: 6.0,
            avg_char_width: 6.0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct FontFace {
    pub family: String,
    pub weight: FontWeight,
    pub style: FontStyle,
    pub data: Vec<u8>,
    pub metrics: FontMetrics,
}

pub struct FontRegistry {
    fonts: HashMap<String, Vec<FontFace>>,
    system_fonts: HashMap<String, FontMetrics>,
}

impl FontRegistry {
    pub fn new() -> Self {
        let mut system_fonts = HashMap::new();
        system_fonts.insert("sans-serif".to_string(), FontMetrics::default());
        system_fonts.insert("serif".to_string(), FontMetrics::default());
        system_fonts.insert(
            "monospace".to_string(),
            FontMetrics {
                avg_char_width: 8.0,
                ..Default::default()
            },
        );
        FontRegistry {
            fonts: HashMap::new(),
            system_fonts,
        }
    }

    pub fn register(&mut self, face: FontFace) {
        self.fonts
            .entry(face.family.clone())
            .or_insert_with(Vec::new)
            .push(face);
    }

    pub fn find(&self, family: &str, weight: FontWeight, style: FontStyle) -> Option<&FontFace> {
        self.fonts
            .get(family)?
            .iter()
            .find(|f| f.weight == weight && f.style == style)
            .or_else(|| self.fonts.get(family)?.first())
    }

    pub fn get_metrics(&self, family: &str) -> FontMetrics {
        self.system_fonts.get(family).cloned().unwrap_or_default()
    }
}
