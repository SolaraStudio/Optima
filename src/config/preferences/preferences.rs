#[derive(Debug, Clone)]
pub struct Preferences {
    pub font_family: String,
    pub font_size: f32,
    pub language: String,
    pub dark_mode: bool,
    pub smooth_scrolling: bool,
    pub user_agent: String,
}

impl Default for Preferences {
    fn default() -> Self {
        Preferences {
            font_family: "sans-serif".to_string(),
            font_size: 16.0,
            language: "en".to_string(),
            dark_mode: false,
            smooth_scrolling: true,
            user_agent: format!("Optima/{}", env!("CARGO_PKG_VERSION")),
        }
    }
}

impl Preferences {
    pub fn new() -> Self { Self::default() }
}
