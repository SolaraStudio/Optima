use font_kit::font::Font;
use font_kit::source::SystemSource;
use font_kit::properties::{Properties, Weight, Stretch, Style};
use std::collections::HashMap;

pub struct FontLoader;

impl FontLoader {
    pub fn load(name: &str) -> Option<Font> {
        let source = SystemSource::new();
        source
            .select_best_match(&[name], &Properties::default())
            .ok()
            .and_then(|f| f.load().ok())
    }

    pub fn load_default() -> Option<Font> {
        Self::load("sans-serif")
    }

    pub fn load_with_properties(name: &str, weight: Weight, stretch: Stretch, style: Style) -> Option<Font> {
        let source = SystemSource::new();
        let props = Properties::new()
            .weight(weight)
            .stretch(stretch)
            .style(style);
        source
            .select_best_match(&[name], &props)
            .ok()
            .and_then(|f| f.load().ok())
    }

    pub fn load_system_fonts() -> Vec<String> {
        let source = SystemSource::new();
        source
            .all_fonts()
            .unwrap_or_default()
            .iter()
            .filter_map(|f| f.load().ok())
            .filter_map(|f| f.full_name().ok())
            .collect()
    }

    pub fn load_families() -> Vec<String> {
        let source = SystemSource::new();
        source
            .all_fonts()
            .unwrap_or_default()
            .iter()
            .filter_map(|f| f.load().ok())
            .filter_map(|f| f.family_name().ok())
            .collect()
    }

    pub fn load_path(name: &str) -> Option<String> {
        let source = SystemSource::new();
        source
            .select_best_match(&[name], &Properties::default())
            .ok()
            .and_then(|f| f.path().ok())
            .map(|p| p.to_str().unwrap_or("").to_string())
    }
}

pub struct FontCache {
    cache: HashMap<String, Font>,
}

impl FontCache {
    pub fn new() -> Self {
        Self {
            cache: HashMap::new(),
        }
    }

    pub fn get(&mut self, name: &str) -> Option<&Font> {
        if self.cache.contains_key(name) {
            return self.cache.get(name);
        }
        if let Some(font) = FontLoader::load(name) {
            self.cache.insert(name.to_string(), font);
            return self.cache.get(name);
        }
        None
    }

    pub fn get_default(&mut self) -> Option<&Font> {
        self.get("sans-serif")
    }

    pub fn clear(&mut self) {
        self.cache.clear();
    }
}
