use crate::api::config::EngineConfig;

pub struct PublicApi {
    version: String,
}

impl Default for PublicApi {
    fn default() -> Self {
        Self::new()
    }
}

impl PublicApi {
    pub fn new() -> Self {
        PublicApi {
            version: env!("CARGO_PKG_VERSION").to_string(),
        }
    }

    pub fn version(&self) -> &str {
        &self.version
    }

    pub fn init(&self, _config: EngineConfig) -> Result<(), String> {
        Ok(())
    }
}
