use std::collections::HashMap;
use std::time::{Duration, Instant};

pub struct CacheEntry {
    pub data: Vec<u8>,
    pub expires_at: Instant,
}

pub struct Cache {
    store: HashMap<String, CacheEntry>,
    ttl: Duration,
}

impl Cache {
    pub fn new(ttl_seconds: u64) -> Self {
        Self {
            store: HashMap::new(),
            ttl: Duration::from_secs(ttl_seconds),
        }
    }

    pub fn get(&self, key: &str) -> Option<&Vec<u8>> {
        if let Some(entry) = self.store.get(key) {
            if entry.expires_at > Instant::now() {
                return Some(&entry.data);
            }
        }
        None
    }

    pub fn put(&mut self, key: &str, data: Vec<u8>) {
        let expires_at = Instant::now() + self.ttl;
        self.store.insert(key.to_string(), CacheEntry { data, expires_at });
    }

    pub fn remove(&mut self, key: &str) {
        self.store.remove(key);
    }

    pub fn clear(&mut self) {
        self.store.clear();
    }
}
