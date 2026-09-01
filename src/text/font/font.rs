use fontdue::Font;
use std::collections::HashMap;
use std::sync::Arc;

pub struct FontLoader {
    fonts: HashMap<String, Arc<Font>>,
}

impl FontLoader {
    pub fn new() -> Self {
        Self {
            fonts: HashMap::new(),
        }
    }

    pub fn load_from_bytes(&mut self, name: &str, data: &[u8]) -> Option<Arc<Font>> {
        if self.fonts.contains_key(name) {
            return self.fonts.get(name).cloned();
        }
        if let Ok(font) = Font::from_bytes(data, fontdue::FontSettings::default()) {
            let arc = Arc::new(font);
            self.fonts.insert(name.to_string(), arc.clone());
            return Some(arc);
        }
        None
    }

    pub fn load_from_file(&mut self, name: &str, path: &str) -> Option<Arc<Font>> {
        let data = std::fs::read(path).ok()?;
        self.load_from_bytes(name, &data)
    }

    pub fn load_from_url(&mut self, name: &str, url: &str) -> Option<Arc<Font>> {
        if let Ok(response) = reqwest::blocking::get(url) {
            if let Ok(bytes) = response.bytes() {
                return self.load_from_bytes(name, &bytes);
            }
        }
        None
    }

    pub fn get(&self, name: &str) -> Option<Arc<Font>> {
        self.fonts.get(name).cloned()
    }

    pub fn get_all(&self) -> Vec<(&String, &Arc<Font>)> {
        self.fonts.iter().map(|(k, v)| (k, v)).collect()
    }

    pub fn clear(&mut self) {
        self.fonts.clear();
    }

    pub fn load_default(&mut self) -> Option<Arc<Font>> {
        // Embed a small fallback font (e.g., DejaVu Sans) as a static array.
        // For now, return None.
        None
    }
}

impl Default for FontLoader {
    fn default() -> Self {
        Self::new()
    }
}
