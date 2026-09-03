use std::collections::HashMap;

pub struct LocalHost {
    assets: HashMap<String, Vec<u8>>,
    content_types: HashMap<String, String>,
}

impl LocalHost {
    pub fn new() -> Self {
        LocalHost {
            assets: HashMap::new(),
            content_types: HashMap::new(),
        }
    }

    pub fn register_asset(&mut self, path: &str, content_type: &str, data: Vec<u8>) {
        let key = Self::normalize_path(path);
        self.content_types
            .insert(key.clone(), content_type.to_string());
        self.assets.insert(key, data);
    }

    pub fn register_text(&mut self, path: &str, content_type: &str, text: &str) {
        self.register_asset(path, content_type, text.as_bytes().to_vec());
    }

    pub fn get_asset(&self, path: &str) -> Option<(&[u8], &str)> {
        let key = Self::normalize_path(path);
        let data = self.assets.get(&key)?;
        let ct = self
            .content_types
            .get(&key)
            .map(|s| s.as_str())
            .unwrap_or("application/octet-stream");
        Some((data, ct))
    }

    pub fn has_asset(&self, path: &str) -> bool {
        self.assets.contains_key(&Self::normalize_path(path))
    }

    pub fn remove_asset(&mut self, path: &str) -> bool {
        let key = Self::normalize_path(path);
        self.content_types.remove(&key);
        self.assets.remove(&key).is_some()
    }

    pub fn clear(&mut self) {
        self.assets.clear();
        self.content_types.clear();
    }

    pub fn asset_count(&self) -> usize {
        self.assets.len()
    }

    pub fn paths(&self) -> Vec<&str> {
        self.assets.keys().map(|s| s.as_str()).collect()
    }

    pub fn is_localhost_url(url: &str) -> bool {
        url.starts_with("http://localhost")
            || url.starts_with("http://127.0.0.1")
            || url.starts_with("http://[::1]")
            || url.starts_with("https://localhost")
            || url.starts_with("https://127.0.0.1")
    }

    pub fn resolve_path(url: &str) -> Option<String> {
        if !Self::is_localhost_url(url) {
            return None;
        }
        let rest = if let Some(pos) = url.find("://") {
            &url[pos + 3..]
        } else {
            return None;
        };
        let rest = if let Some(pos) = rest.find('/') {
            &rest[pos..]
        } else {
            "/"
        };
        Some(Self::normalize_path(rest))
    }

    fn normalize_path(path: &str) -> String {
        let mut normalized = path.to_string();
        if normalized.starts_with('/') {
            normalized.remove(0);
        }
        if normalized.is_empty() {
            normalized = "index.html".to_string();
        }
        normalized
    }
}

impl Default for LocalHost {
    fn default() -> Self {
        Self::new()
    }
}
