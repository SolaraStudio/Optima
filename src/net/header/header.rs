use std::collections::HashMap;

#[derive(Debug, Clone, Default)]
pub struct Headers {
    pub map: HashMap<String, String>,
}

impl Headers {
    pub fn new() -> Self { Self::default() }

    pub fn get(&self, name: &str) -> Option<&String> {
        self.map.get(&name.to_lowercase())
    }

    pub fn set(&mut self, name: &str, value: &str) {
        self.map.insert(name.to_lowercase(), value.to_string());
    }

    pub fn remove(&mut self, name: &str) -> Option<String> {
        self.map.remove(&name.to_lowercase())
    }

    pub fn has(&self, name: &str) -> bool {
        self.map.contains_key(&name.to_lowercase())
    }

    pub fn content_type(&self) -> Option<&String> { self.get("content-type") }
    pub fn content_length(&self) -> Option<usize> {
        self.get("content-length").and_then(|v| v.parse().ok())
    }

    pub fn from_str(s: &str) -> Self {
        let mut headers = Headers::new();
        for line in s.lines() {
            if let Some((name, value)) = line.split_once(':') {
                headers.set(name.trim(), value.trim());
            }
        }
        headers
    }
}
