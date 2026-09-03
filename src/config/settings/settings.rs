use crate::config::debug::DebugConfig;
use crate::config::env::EnvConfig;
use crate::config::feature::FeatureFlags;
use crate::config::preferences::Preferences;
use crate::config::release::ReleaseConfig;

#[derive(Debug, Clone)]
pub struct Settings {
    pub debug: DebugConfig,
    pub env: EnvConfig,
    pub features: FeatureFlags,
    pub preferences: Preferences,
    pub release: ReleaseConfig,
    pub width: u32,
    pub height: u32,
    pub device_pixel_ratio: f32,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            debug: DebugConfig::default(),
            env: EnvConfig::default(),
            features: FeatureFlags::default(),
            preferences: Preferences::default(),
            release: ReleaseConfig::default(),
            width: 800,
            height: 600,
            device_pixel_ratio: 1.0,
        }
    }
}

impl Settings {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_size(mut self, w: u32, h: u32) -> Self {
        self.width = w;
        self.height = h;
        self
    }
    pub fn with_dpr(mut self, dpr: f32) -> Self {
        self.device_pixel_ratio = dpr;
        self
    }
}
