#[derive(Debug, Clone)]
pub struct Compat {
    pub engine_version: String,
    pub target_platform: Platform,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Platform {
    Android,
    Desktop,
    Web,
    Unknown,
}

impl Compat {
    pub fn new() -> Self {
        Compat {
            engine_version: env!("CARGO_PKG_VERSION").to_string(),
            target_platform: Self::detect_platform(),
        }
    }

    fn detect_platform() -> Platform {
        if cfg!(target_os = "android") {
            Platform::Android
        } else if cfg!(target_os = "linux")
            || cfg!(target_os = "macos")
            || cfg!(target_os = "windows")
        {
            Platform::Desktop
        } else {
            Platform::Unknown
        }
    }

    pub fn is_compatible(&self, min_version: &str) -> bool {
        self.engine_version.as_str() >= min_version
    }
}
