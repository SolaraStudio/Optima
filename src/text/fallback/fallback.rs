use std::collections::HashMap;

pub struct FontFallback {
    fallback_chain: Vec<String>,
    char_font_cache: HashMap<char, String>,
}

impl FontFallback {
    pub fn new() -> Self {
        Self {
            fallback_chain: vec![
                "sans-serif".to_string(),
                "serif".to_string(),
                "monospace".to_string(),
                "cursive".to_string(),
                "fantasy".to_string(),
            ],
            char_font_cache: HashMap::new(),
        }
    }

    pub fn get_font_for_char(&mut self, c: char, preferred: &str) -> String {
        if let Some(font) = self.char_font_cache.get(&c) {
            return font.clone();
        }

        if Self::is_char_supported(c, preferred) {
            self.char_font_cache.insert(c, preferred.to_string());
            return preferred.to_string();
        }

        for fallback in &self.fallback_chain {
            if Self::is_char_supported(c, fallback) {
                self.char_font_cache.insert(c, fallback.clone());
                return fallback.clone();
            }
        }

        self.char_font_cache.insert(c, "sans-serif".to_string());
        "sans-serif".to_string()
    }

    pub fn is_char_supported(c: char, font_name: &str) -> bool {
        // Simplified: assume all fonts support ASCII
        if c.is_ascii() {
            return true;
        }
        // For non-ASCII, use a basic heuristic
        match c {
            'あ' | 'い' | 'う' | 'え' | 'お' | 'か' | 'き' | 'く' | 'け' | 'こ' => true,
            '一' | '二' | '三' | '四' | '五' | '六' | '七' | '八' | '九' | '十' => true,
            'α' | 'β' | 'γ' | 'δ' | 'ε' | 'ζ' | 'η' | 'θ' | 'ι' | 'κ' => true,
            'А' | 'Б' | 'В' | 'Г' | 'Д' | 'Е' | 'Ё' | 'Ж' | 'З' | 'И' => true,
            _ => false,
        }
    }

    pub fn add_fallback(&mut self, font: &str) {
        if !self.fallback_chain.contains(&font.to_string()) {
            self.fallback_chain.push(font.to_string());
        }
    }

    pub fn get_fallback_chain(&self) -> &Vec<String> {
        &self.fallback_chain
    }

    pub fn clear_cache(&mut self) {
        self.char_font_cache.clear();
    }
}
