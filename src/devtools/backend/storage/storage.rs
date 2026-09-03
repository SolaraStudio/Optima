use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct StorageEntry {
    pub key: String,
    pub value: String,
    pub domain: String,
    pub secure: bool,
    pub http_only: bool,
}

pub struct StorageBackend {
    pub cookies: Vec<StorageEntry>,
    pub local_storage: HashMap<String, String>,
    pub session_storage: HashMap<String, String>,
}

impl StorageBackend {
    pub fn new() -> Self {
        StorageBackend {
            cookies: Vec::new(),
            local_storage: HashMap::new(),
            session_storage: HashMap::new(),
        }
    }

    pub fn add_cookie(
        &mut self,
        key: &str,
        value: &str,
        domain: &str,
        secure: bool,
        http_only: bool,
    ) {
        self.cookies.push(StorageEntry {
            key: key.to_string(),
            value: value.to_string(),
            domain: domain.to_string(),
            secure,
            http_only,
        });
    }

    pub fn get_cookies(&self) -> &[StorageEntry] {
        &self.cookies
    }

    pub fn set_local_storage(&mut self, key: &str, value: &str) {
        self.local_storage
            .insert(key.to_string(), value.to_string());
    }

    pub fn get_local_storage(&self, key: &str) -> Option<&String> {
        self.local_storage.get(key)
    }

    pub fn get_all_local_storage(&self) -> &HashMap<String, String> {
        &self.local_storage
    }

    pub fn remove_local_storage(&mut self, key: &str) {
        self.local_storage.remove(key);
    }

    pub fn clear_local_storage(&mut self) {
        self.local_storage.clear();
    }

    pub fn set_session_storage(&mut self, key: &str, value: &str) {
        self.session_storage
            .insert(key.to_string(), value.to_string());
    }

    pub fn get_session_storage(&self, key: &str) -> Option<&String> {
        self.session_storage.get(key)
    }

    pub fn get_all_session_storage(&self) -> &HashMap<String, String> {
        &self.session_storage
    }

    pub fn remove_session_storage(&mut self, key: &str) {
        self.session_storage.remove(key);
    }

    pub fn clear_session_storage(&mut self) {
        self.session_storage.clear();
    }

    pub fn to_json(&self) -> Value {
        let cookies: Vec<Value> = self
            .cookies
            .iter()
            .map(|c| {
                serde_json::json!({
                    "key": c.key,
                    "value": c.value,
                    "domain": c.domain,
                    "secure": c.secure,
                    "httpOnly": c.http_only
                })
            })
            .collect();
        let local: Vec<Value> = self
            .local_storage
            .iter()
            .map(|(k, v)| serde_json::json!({ "key": k, "value": v }))
            .collect();
        let session: Vec<Value> = self
            .session_storage
            .iter()
            .map(|(k, v)| serde_json::json!({ "key": k, "value": v }))
            .collect();
        serde_json::json!({
            "cookies": cookies,
            "localStorage": local,
            "sessionStorage": session
        })
    }
}

impl Default for StorageBackend {
    fn default() -> Self {
        Self::new()
    }
}
