use std::collections::HashMap;
use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
pub struct CacheEntry {
    pub url: String,
    pub data: Vec<u8>,
    pub headers: HashMap<String, String>,
    pub inserted_at: Instant,
    pub max_age: Duration,
}

impl CacheEntry {
    pub fn is_expired(&self) -> bool { self.inserted_at.elapsed() > self.max_age }
}

pub struct HttpCache {
    entries: HashMap<String, CacheEntry>,
    max_size: usize,
    total_size: usize,
}

impl HttpCache {
    pub fn new(max_size: usize) -> Self {
        HttpCache { entries: HashMap::new(), max_size, total_size: 0 }
    }

    pub fn get(&self, url: &str) -> Option<&CacheEntry> {
        self.entries.get(url).filter(|e| !e.is_expired())
    }

    pub fn insert(&mut self, url: String, data: Vec<u8>, headers: HashMap<String, String>, max_age: Duration) {
        self.total_size += data.len();
        self.entries.insert(url.clone(), CacheEntry {
            url, data, headers, inserted_at: Instant::now(), max_age,
        });
        self.evict();
    }

    fn evict(&mut self) {
        while self.total_size > self.max_size {
            if let Some((oldest_url, oldest_time)) = self.entries.iter()
                .min_by_key(|(_, e)| e.inserted_at)
                .map(|(u, e)| (u.clone(), e.inserted_at))
            {
                if let Some(entry) = self.entries.remove(&oldest_url) {
                    self.total_size -= entry.data.len();
                } else { break; }
            } else { break; }
        }
    }

    pub fn clear(&mut self) { self.entries.clear(); self.total_size = 0; }
    pub fn len(&self) -> usize { self.entries.len() }
}
