pub mod android;
pub mod desktop;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Platform {
    Android,
    Desktop,
    Unknown,
}

pub fn detect_platform() -> Platform {
    #[cfg(target_os = "android")]
    {
        Platform::Android
    }
    #[cfg(not(target_os = "android"))]
    {
        Platform::Desktop
    }
}
