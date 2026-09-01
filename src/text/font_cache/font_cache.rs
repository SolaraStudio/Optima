use super::font::FontLoader;
use fontdue::Font;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

pub struct FontCache {
    loader: Arc<RwLock<FontLoader>>,
    fallback_chain: Vec<Arc<Font>>,
    char_font_cache: HashMap<char, Arc<Font>>,
}

impl FontCache {
    pub fn new() -> Self {
        Self {
            loader: Arc::new(RwLock::new(FontLoader::new())),
            fallback_chain: Vec::new(),
            char_font_cache: HashMap::new(),
        }
    }

    pub fn load_from_bytes(&self, name: &str, data: &[u8]) -> Option<Arc<Font>> {
        let mut loader = self.loader.write().unwrap();
        loader.load_from_bytes(name, data)
    }

    pub fn load_from_file(&self, name: &str, path: &str) -> Option<Arc<Font>> {
        let mut loader = self.loader.write().unwrap();
        loader.load_from_file(name, path)
    }

    pub fn load_from_url(&self, name: &str, url: &str) -> Option<Arc<Font>> {
        let mut loader = self.loader.write().unwrap();
        loader.load_from_url(name, url)
    }

    pub fn get(&self, name: &str) -> Option<Arc<Font>> {
        let loader = self.loader.read().unwrap();
        loader.get(name)
    }

    pub fn add_fallback(&mut self, font: Arc<Font>) {
        self.fallback_chain.push(font);
    }

    pub fn get_fallback_chain(&self) -> &[Arc<Font>] {
        &self.fallback_chain
    }

    pub fn get_font_for_char(&mut self, c: char, preferred_font: Option<&str>) -> Option<Arc<Font>> {
        // Check cache first
        if let Some(font) = self.char_font_cache.get(&c) {
            return Some(font.clone());
        }

        // Try preferred font
        if let Some(name) = preferred_font {
            if let Some(font) = self.get(name) {
                if self.can_render_char(&font, c) {
                    self.char_font_cache.insert(c, font.clone());
                    return Some(font);
                }
            }
        }

        // Try fallback chain
        for font in &self.fallback_chain {
            if self.can_render_char(font, c) {
                self.char_font_cache.insert(c, font.clone());
                return Some(font.clone());
            }
        }

        // Try any loaded font
        let loader = self.loader.read().unwrap();
        for (_, font) in loader.get_all() {
            if self.can_render_char(font, c) {
                self.char_font_cache.insert(c, font.clone());
                return Some(font.clone());
            }
        }

        // If still not found, return None (will use a fallback later)
        None
    }

    fn can_render_char(&self, font: &Font, c: char) -> bool {
        // Attempt to rasterize; if width/height > 0, it exists.
        let metrics = font.metrics();
        let rasterized = font.rasterize(c, metrics.units_per_em);
        rasterized.width > 0 && rasterized.height > 0
    }

    pub fn clear(&mut self) {
        let mut loader = self.loader.write().unwrap();
        loader.clear();
        self.fallback_chain.clear();
        self.char_font_cache.clear();
    }

    pub fn load_default_fallback(&mut self, font: Arc<Font>) {
        self.add_fallback(font);
    }
}

impl Default for FontCache {
    fn default() -> Self {
        Self::new()
    }
}
