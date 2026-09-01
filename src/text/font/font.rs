use fontdue::Font;
use std::collections::HashMap;

pub struct FontLoader {
    fonts: HashMap<String, Font>,
}

impl FontLoader {
    pub fn new() -> Self {
        Self {
            fonts: HashMap::new(),
        }
    }

    pub fn load(&mut self, name: &str, data: &[u8]) -> Option<&Font> {
        if self.fonts.contains_key(name) {
            return self.fonts.get(name);
        }
        if let Ok(font) = Font::from_bytes(data, fontdue::FontSettings::default()) {
            self.fonts.insert(name.to_string(), font);
            return self.fonts.get(name);
        }
        None
    }

    pub fn load_default(&mut self) -> Option<&Font> {
        // Try to load a built-in fallback font (e.g., a small TTF embedded as a constant)
        // For now, return None – we'll need to embed a font or load from assets later.
        None
    }

    pub fn get(&self, name: &str) -> Option<&Font> {
        self.fonts.get(name)
    }

    pub fn clear(&mut self) {
        self.fonts.clear();
    }
}
