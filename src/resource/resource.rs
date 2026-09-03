use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum ResourceType {
    Image,
    Font,
    Script,
    Style,
    Audio,
    Video,
    Document,
    Other(String),
}

#[derive(Debug, Clone)]
pub struct Resource {
    pub id: u64,
    pub url: String,
    pub resource_type: ResourceType,
    pub data: Vec<u8>,
    pub loaded: bool,
}

pub struct ResourceManager {
    resources: HashMap<u64, Resource>,
    next_id: u64,
}

impl ResourceManager {
    pub fn new() -> Self {
        ResourceManager { resources: HashMap::new(), next_id: 1 }
    }

    pub fn insert(&mut self, url: &str, resource_type: ResourceType, data: Vec<u8>) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        self.resources.insert(id, Resource {
            id, url: url.to_string(), resource_type, data, loaded: true,
        });
        id
    }

    pub fn get(&self, id: u64) -> Option<&Resource> { self.resources.get(&id) }
    pub fn get_by_url(&self, url: &str) -> Option<&Resource> {
        self.resources.values().find(|r| r.url == url)
    }
    pub fn remove(&mut self, id: u64) -> Option<Resource> { self.resources.remove(&id) }
    pub fn clear(&mut self) { self.resources.clear(); }
    pub fn len(&self) -> usize { self.resources.len() }
    pub fn is_empty(&self) -> bool { self.resources.is_empty() }
}
