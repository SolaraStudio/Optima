#[derive(Debug, Clone)]
pub struct ReleaseConfig {
    pub version: String,
    pub channel: String,
    pub build_id: String,
}

impl Default for ReleaseConfig {
    fn default() -> Self {
        ReleaseConfig {
            version: env!("CARGO_PKG_VERSION").to_string(),
            channel: "stable".to_string(),
            build_id: String::new(),
        }
    }
}

impl ReleaseConfig {
    pub fn new() -> Self { Self::default() }
}
