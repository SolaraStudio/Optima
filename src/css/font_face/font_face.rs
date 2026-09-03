#[derive(Debug, Clone)]
pub struct FontFaceRule {
    pub family: String,
    pub src: String,
    pub weight: Option<String>,
    pub style: Option<String>,
    pub stretch: Option<String>,
    pub unicode_range: Option<String>,
}

impl FontFaceRule {
    pub fn new(family: &str, src: &str) -> Self {
        FontFaceRule {
            family: family.to_string(),
            src: src.to_string(),
            weight: None,
            style: None,
            stretch: None,
            unicode_range: None,
        }
    }

    pub fn with_weight(mut self, weight: &str) -> Self {
        self.weight = Some(weight.to_string());
        self
    }

    pub fn with_style(mut self, style: &str) -> Self {
        self.style = Some(style.to_string());
        self
    }
}
