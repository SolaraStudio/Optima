pub struct FallbackFontChain {
    pub families: Vec<String>,
}

impl Default for FallbackFontChain {
    fn default() -> Self {
        Self::new()
    }
}

impl FallbackFontChain {
    pub fn new() -> Self {
        FallbackFontChain {
            families: vec![
                "sans-serif".to_string(),
                "serif".to_string(),
                "monospace".to_string(),
            ],
        }
    }

    pub fn add(&mut self, family: &str) {
        if !self.families.contains(&family.to_string()) {
            self.families.push(family.to_string());
        }
    }

    pub fn get_chain(&self) -> &[String] {
        &self.families
    }
}
