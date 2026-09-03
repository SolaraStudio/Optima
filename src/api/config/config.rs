use std::time::Duration;

use crate::config::settings::Settings;

#[derive(Clone)]
pub struct EngineConfig {
    pub user_agent: String,
    pub javascript_enabled: bool,
    pub images_enabled: bool,
    pub css_enabled: bool,
    pub default_font_size: u32,
    pub default_font_family: String,
    pub enable_webgl: bool,
    pub enable_webrtc: bool,
    pub cache_enabled: bool,
    pub cache_size: usize,
    pub timeout: Duration,
    pub max_connections: usize,
    pub allow_file_protocol: bool,
}

impl EngineConfig {
    pub fn new() -> Self {
        EngineConfig {
            user_agent: "Optima/1.0".to_string(),
            javascript_enabled: true,
            images_enabled: true,
            css_enabled: true,
            default_font_size: 16,
            default_font_family: "sans-serif".to_string(),
            enable_webgl: false,
            enable_webrtc: false,
            cache_enabled: true,
            cache_size: 50 * 1024 * 1024,
            timeout: Duration::from_secs(30),
            max_connections: 6,
            allow_file_protocol: false,
        }
    }

    pub fn from_settings(settings: &Settings) -> Self {
        EngineConfig {
            user_agent: settings.preferences.user_agent.clone(),
            javascript_enabled: true,
            images_enabled: settings.features.enable_gpu,
            css_enabled: true,
            default_font_size: settings.preferences.font_size as u32,
            default_font_family: settings.preferences.font_family.clone(),
            enable_webgl: settings.features.enable_webgl,
            enable_webrtc: false,
            cache_enabled: settings.features.enable_fetch,
            cache_size: 50 * 1024 * 1024,
            timeout: Duration::from_secs(30),
            max_connections: 6,
            allow_file_protocol: false,
        }
    }

    pub fn with_user_agent(mut self, ua: &str) -> Self {
        self.user_agent = ua.to_string();
        self
    }

    pub fn with_javascript(mut self, enabled: bool) -> Self {
        self.javascript_enabled = enabled;
        self
    }

    pub fn with_images(mut self, enabled: bool) -> Self {
        self.images_enabled = enabled;
        self
    }

    pub fn with_css(mut self, enabled: bool) -> Self {
        self.css_enabled = enabled;
        self
    }

    pub fn with_font_size(mut self, size: u32) -> Self {
        self.default_font_size = size;
        self
    }

    pub fn with_timeout(mut self, duration: Duration) -> Self {
        self.timeout = duration;
        self
    }
}

impl Default for EngineConfig {
    fn default() -> Self {
        Self::new()
    }
}
