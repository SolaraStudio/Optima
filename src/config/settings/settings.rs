#[derive(Debug, Clone)]
pub struct Settings {
    pub enable_javascript: bool,
    pub enable_media: bool,
    pub enable_cookies: bool,
    pub enable_cache: bool,
    pub enable_webgl: bool,
    pub enable_webrtc: bool,
    pub enable_geolocation: bool,
    pub enable_notifications: bool,
    pub enable_fullscreen: bool,
    pub enable_autoplay: bool,
    pub default_font: String,
    pub default_font_size: f32,
    pub default_encoding: String,
    pub user_agent: String,
    pub home_page: String,
    pub search_engine: String,
    pub download_path: String,
    pub cache_size_mb: u32,
    pub max_tabs: u32,
    pub max_history: u32,
    pub incognito_mode: bool,
    pub do_not_track: bool,
    pub private_browsing: bool,
}

impl Settings {
    pub fn default() -> Self {
        Self {
            enable_javascript: true,
            enable_media: true,
            enable_cookies: true,
            enable_cache: true,
            enable_webgl: true,
            enable_webrtc: false,
            enable_geolocation: false,
            enable_notifications: false,
            enable_fullscreen: true,
            enable_autoplay: false,
            default_font: "sans-serif".to_string(),
            default_font_size: 16.0,
            default_encoding: "UTF-8".to_string(),
            user_agent: "Optima/0.150.10-dev".to_string(),
            home_page: "about:blank".to_string(),
            search_engine: "https://google.com/search?q=".to_string(),
            download_path: "downloads".to_string(),
            cache_size_mb: 100,
            max_tabs: 100,
            max_history: 1000,
            incognito_mode: false,
            do_not_track: true,
            private_browsing: false,
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

    pub fn with_search_engine(mut self, engine: &str) -> Self {
        self.search_engine = engine.to_string();
        self
    }

    pub fn with_font(mut self, font: &str, size: f32) -> Self {
        self.default_font = font.to_string();
        self.default_font_size = size;
        self
    }

    pub fn with_cache_size(mut self, size_mb: u32) -> Self {
        self.cache_size_mb = size_mb;
        self
    }

    pub fn with_max_tabs(mut self, max: u32) -> Self {
        self.max_tabs = max;
        self
    }

    pub fn is_javascript_enabled(&self) -> bool {
        self.enable_javascript
    }

    pub fn is_media_enabled(&self) -> bool {
        self.enable_media
    }

    pub fn is_incognito(&self) -> bool {
        self.incognito_mode || self.private_browsing
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self::default()
    }
}
