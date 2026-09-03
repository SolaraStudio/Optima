use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct SourceFile {
    pub url: String,
    pub content: String,
    pub content_type: String,
    pub last_modified: u64,
}

pub struct SourcesBackend {
    pub sources: HashMap<String, SourceFile>,
}

impl SourcesBackend {
    pub fn new() -> Self {
        SourcesBackend {
            sources: HashMap::new(),
        }
    }

    pub fn add_source(&mut self, url: &str, content: &str, content_type: &str) {
        let source = SourceFile {
            url: url.to_string(),
            content: content.to_string(),
            content_type: content_type.to_string(),
            last_modified: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_millis() as u64,
        };
        self.sources.insert(url.to_string(), source);
    }

    pub fn get_source(&self, url: &str) -> Option<&SourceFile> {
        self.sources.get(url)
    }

    pub fn get_sources(&self) -> Vec<&SourceFile> {
        self.sources.values().collect()
    }

    pub fn remove_source(&mut self, url: &str) {
        self.sources.remove(url);
    }

    pub fn clear(&mut self) {
        self.sources.clear();
    }

    pub fn to_json(&self) -> Value {
        let sources: Vec<Value> = self
            .sources
            .values()
            .map(|s| {
                serde_json::json!({
                    "url": s.url,
                    "contentType": s.content_type,
                    "lastModified": s.last_modified
                })
            })
            .collect();
        serde_json::json!({ "sources": sources })
    }

    pub fn get_content(&self, url: &str) -> Option<String> {
        self.sources.get(url).map(|s| s.content.clone())
    }
}

impl Default for SourcesBackend {
    fn default() -> Self {
        Self::new()
    }
}
