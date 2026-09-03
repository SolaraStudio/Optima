use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct DnsEntry {
    pub host: String,
    pub addresses: Vec<String>,
    pub ttl: u32,
}

pub struct DnsResolver {
    cache: HashMap<String, DnsEntry>,
}

impl Default for DnsResolver {
    fn default() -> Self {
        Self::new()
    }
}

impl DnsResolver {
    pub fn new() -> Self {
        DnsResolver {
            cache: HashMap::new(),
        }
    }

    pub fn resolve(&self, host: &str) -> Option<&DnsEntry> {
        self.cache.get(host)
    }

    pub fn cache_entry(&mut self, entry: DnsEntry) {
        self.cache.insert(entry.host.clone(), entry);
    }

    pub fn remove(&mut self, host: &str) {
        self.cache.remove(host);
    }
    pub fn clear(&mut self) {
        self.cache.clear();
    }
}
