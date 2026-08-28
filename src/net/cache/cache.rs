use std::collections::HashMap;
use std::time::{Duration, Instant};

pub struct CacheEntry {
    pub data: Vec<u8>,
    pub expires_at: Instant,
    pub etag: Option<String>,
    pub last_modified: Option<String>,
}

pub struct Cache {
    store: HashMap<String, CacheEntry>,
    ttl: Duration,
    max_size: usize,
}

impl Cache {
    pub fn new(ttl_seconds: u64, max_size: usize) -> Self {
        Self {
            store: HashMap::new(),
            ttl: Duration::from_secs(ttl_seconds),
            max_size,
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

    pub fn get_entry(&self, key: &str) -> Option<&CacheEntry> {
        if let Some(entry) = self.store.get(key) {
            if entry.expires_at > Instant::now() {
                return Some(entry);
            }
        }
        None
    }

    pub fn get_mut_entry(&mut self, key: &str) -> Option<&mut CacheEntry> {
        if let Some(entry) = self.store.get_mut(key) {
            if entry.expires_at > Instant::now() {
                return Some(entry);
            }
        }
        None
    }

    pub fn put(&mut self, key: &str, data: Vec<u8>) {
        self.put_with_ttl(key, data, self.ttl)
    }

    pub fn put_with_ttl(&mut self, key: &str, data: Vec<u8>, ttl: Duration) {
        if self.store.len() >= self.max_size {
            self.evict_oldest();
        }
        let expires_at = Instant::now() + ttl;
        self.store.insert(
            key.to_string(),
            CacheEntry {
                data,
                expires_at,
                etag: None,
                last_modified: None,
            },
        );
    }

    pub fn put_with_metadata(&mut self, key: &str, data: Vec<u8>, etag: Option<String>, last_modified: Option<String>) {
        if self.store.len() >= self.max_size {
            self.evict_oldest();
        }
        let expires_at = Instant::now() + self.ttl;
        self.store.insert(
            key.to_string(),
            CacheEntry {
                data,
                expires_at,
                etag,
                last_modified,
            },
        );
    }

    pub fn remove(&mut self, key: &str) {
        self.store.remove(key);
    }

    pub fn clear(&mut self) {
        self.store.clear();
    }

    pub fn contains(&self, key: &str) -> bool {
        if let Some(entry) = self.store.get(key) {
            if entry.expires_at > Instant::now() {
                return true;
            }
        }
        false
    }

    pub fn evict_oldest(&mut self) {
        if let Some(key) = self.store
            .iter()
            .min_by_key(|(_, entry)| entry.expires_at)
            .map(|(k, _)| k.clone())
        {
            self.store.remove(&key);
        }
    }

    pub fn evict_expired(&mut self) {
        let now = Instant::now();
        self.store.retain(|_, entry| entry.expires_at > now);
    }

    pub fn len(&self) -> usize {
        self.store.len()
    }

    pub fn is_empty(&self) -> bool {
        self.store.is_empty()
    }

    pub fn set_ttl(&mut self, ttl: Duration) {
        self.ttl = ttl;
    }

    pub fn set_max_size(&mut self, max_size: usize) {
        self.max_size = max_size;
        while self.store.len() > self.max_size {
            self.evict_oldest();
        }
    }
}

impl Default for Cache {
    fn default() -> Self {
        Self::new(300, 100)
    }
}
