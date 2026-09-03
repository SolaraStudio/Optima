use std::env;

#[derive(Debug, Clone)]
pub struct EnvConfig {
    pub data_dir: String,
    pub cache_dir: String,
    pub temp_dir: String,
    pub is_headless: bool,
}

impl Default for EnvConfig {
    fn default() -> Self {
        let temp = env::temp_dir().to_string_lossy().to_string();
        EnvConfig {
            data_dir: format!("{}/optima", temp),
            cache_dir: format!("{}/optima/cache", temp),
            temp_dir: format!("{}/optima/tmp", temp),
            is_headless: false,
        }
    }
}

impl EnvConfig {
    pub fn from_env() -> Self {
        let mut cfg = Self::default();
        if let Ok(val) = env::var("OPTIMA_DATA_DIR") { cfg.data_dir = val; }
        if let Ok(val) = env::var("OPTIMA_CACHE_DIR") { cfg.cache_dir = val; }
        if let Ok(val) = env::var("OPTIMA_HEADLESS") { cfg.is_headless = val == "1" || val == "true"; }
        cfg
    }
}
