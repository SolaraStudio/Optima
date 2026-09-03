#[derive(Debug, Clone, Default)]
pub struct Body {
    pub data: Vec<u8>,
    pub content_type: Option<String>,
}

impl Body {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn from_bytes(data: Vec<u8>) -> Self {
        Body {
            data,
            content_type: None,
        }
    }
    pub fn from_string(s: &str) -> Self {
        Body {
            data: s.as_bytes().to_vec(),
            content_type: Some("text/plain".to_string()),
        }
    }
    pub fn from_html(s: &str) -> Self {
        Body {
            data: s.as_bytes().to_vec(),
            content_type: Some("text/html".to_string()),
        }
    }
    pub fn from_json(s: &str) -> Self {
        Body {
            data: s.as_bytes().to_vec(),
            content_type: Some("application/json".to_string()),
        }
    }
    pub fn len(&self) -> usize {
        self.data.len()
    }
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
    pub fn text(&self) -> String {
        String::from_utf8_lossy(&self.data).to_string()
    }
    pub fn bytes(&self) -> &[u8] {
        &self.data
    }
    pub fn with_content_type(mut self, ct: &str) -> Self {
        self.content_type = Some(ct.to_string());
        self
    }
}
