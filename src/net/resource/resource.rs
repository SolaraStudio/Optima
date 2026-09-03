use crate::net::header::Headers;

#[derive(Debug, Clone, PartialEq)]
pub enum ResourcePriority {
    Lowest,
    Low,
    Normal,
    High,
    Highest,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ResourceType {
    Document,
    Stylesheet,
    Script,
    Image,
    Font,
    Media,
    WebSocket,
    Other,
}

#[derive(Debug, Clone)]
pub struct ResourceRequest {
    pub url: String,
    pub resource_type: ResourceType,
    pub priority: ResourcePriority,
    pub headers: Headers,
    pub is_blocking: bool,
}

impl ResourceRequest {
    pub fn new(url: &str, resource_type: ResourceType) -> Self {
        ResourceRequest {
            url: url.to_string(), resource_type, priority: ResourcePriority::Normal,
            headers: Headers::new(), is_blocking: false,
        }
    }
}
