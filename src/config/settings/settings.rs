#[derive(Debug, Clone)]
pub struct Settings {
    pub enable_javascript: bool,
    pub enable_media: bool,
    pub enable_cookies: bool,
    pub enable_cache: bool,
    pub default_font: String,
    pub default_font_size: f32,
    pub user_agent: String,
    pub home_page: String,
    pub search_engine: String,
}

impl Settings {
    pub fn default() -> Self {
        Self {
            enable_javascript: true,
            enable_media: true,
            enable_cookies: true,
            enable_cache: true,
            default_font: "sans-serif".to_string(),
            default_font_size: 16.0,
            user_agent: "Optima/0.150.10-dev".to_string(),
            home_page: "about:blank".to_string(),
            search_engine: "https://google.com/search?q=".to_string(),
        }
    }

    pub fn with_user_agent(mut self, ua: &str) -> Self {
        self.user_agent = ua.to_string();
        self
    }

    pub fn with_home_page(mut self, page: &str) -> Self {
        self.home_page = page.to_string();
        self
    }
}
