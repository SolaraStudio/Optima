pub struct PlatformInfo {
    pub os: String,
    pub arch: String,
    pub name: String,
}

impl PlatformInfo {
    pub fn detect() -> Self {
        PlatformInfo {
            os: std::env::consts::OS.to_string(),
            arch: std::env::consts::ARCH.to_string(),
            name: Self::os_name().to_string(),
        }
    }

    fn os_name() -> &'static str {
        if cfg!(target_os = "android") { "Android" }
        else if cfg!(target_os = "linux") { "Linux" }
        else if cfg!(target_os = "macos") { "macOS" }
        else if cfg!(target_os = "windows") { "Windows" }
        else { "Unknown" }
    }

    pub fn is_mobile(&self) -> bool { self.os == "Android" || self.os == "iOS" }
    pub fn is_desktop(&self) -> bool { !self.is_mobile() }
}
