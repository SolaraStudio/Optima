use std::collections::{HashMap, VecDeque};
use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
pub struct CachedResponse {
    pub url: String,
    pub body: Vec<u8>,
    pub headers: HashMap<String, String>,
    pub cached_at: Instant,
    pub expires_at: Instant,
}

impl CachedResponse {
    pub fn is_fresh(&self) -> bool {
        self.expires_at > Instant::now()
    }
}

pub struct TtlCache {
    entries: HashMap<String, CachedResponse>,
    lru_order: VecDeque<String>,
    max_entries: usize,
    default_ttl: Duration,
}

impl TtlCache {
    pub fn new(max_entries: usize, default_ttl: Duration) -> Self {
        TtlCache {
            entries: HashMap::new(),
            lru_order: VecDeque::new(),
            max_entries,
            default_ttl,
        }
    }

    pub fn get(&mut self, url: &str) -> Option<&CachedResponse> {
        if !self.entries.contains_key(url) {
            return None;
        }
        let entry = self.entries.get(url)?;
        if entry.is_fresh() {
            self.touch(url);
            self.entries.get(url)
        } else {
            self.remove(url);
            None
        }
    }

    pub fn insert(&mut self, url: String, body: Vec<u8>, headers: HashMap<String, String>, ttl: Duration) {
        let cached_at = Instant::now();
        let expires_at = cached_at + ttl;
        let entry = CachedResponse { url: url.clone(), body, headers, cached_at, expires_at };
        self.entries.insert(url.clone(), entry);
        self.touch(&url);
        self.evict_lru();
    }

    pub fn insert_default_ttl(&mut self, url: &str, body: Vec<u8>, headers: HashMap<String, String>) {
        self.insert(url.to_string(), body, headers, self.default_ttl);
    }

    pub fn remove(&mut self, url: &str) -> Option<CachedResponse> {
        if let Some(pos) = self.lru_order.iter().position(|u| u == url) {
            self.lru_order.remove(pos);
        }
        self.entries.remove(url)
    }

    fn touch(&mut self, url: &str) {
        if let Some(pos) = self.lru_order.iter().position(|u| u == url) {
            self.lru_order.remove(pos);
        }
        self.lru_order.push_back(url.to_string());
    }

    fn evict_lru(&mut self) {
        while self.entries.len() > self.max_entries {
            if let Some(oldest) = self.lru_order.pop_front() {
                if let Some(entry) = self.entries.get(&oldest) {
                    if !entry.is_fresh() {
                        self.entries.remove(&oldest);
                        continue;
                    }
                }
                self.entries.remove(&oldest);
            } else {
                break;
            }
        }
    }

    pub fn purge_expired(&mut self) {
        let expired: Vec<String> = self.entries.iter()
            .filter(|(_, e)| !e.is_fresh())
            .map(|(k, _)| k.clone())
            .collect();
        for url in expired {
            self.remove(&url);
        }
    }

    pub fn clear(&mut self) {
        self.entries.clear();
        self.lru_order.clear();
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}

impl Default for TtlCache {
    fn default() -> Self {
        TtlCache::new(100, Duration::from_secs(60))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insert_and_get() {
        let mut cache = TtlCache::new(10, Duration::from_secs(60));
        cache.insert_default_ttl("https://a.com", vec![1, 2, 3], HashMap::new());
        let entry = cache.get("https://a.com").unwrap();
        assert_eq!(entry.body, vec![1, 2, 3]);
        assert!(entry.is_fresh());
    }

    #[test]
    fn expired_entry_not_returned() {
        let mut cache = TtlCache::new(10, Duration::from_secs(1));
        cache.insert("https://a.com".to_string(), vec![1], HashMap::new(), Duration::from_millis(1));
        std::thread::sleep(Duration::from_millis(5));
        assert!(cache.get("https://a.com").is_none());
    }

    #[test]
    fn lru_eviction() {
        let mut cache = TtlCache::new(2, Duration::from_secs(60));
        cache.insert_default_ttl("https://a.com", vec![1], HashMap::new());
        cache.insert_default_ttl("https://b.com", vec![2], HashMap::new());
        cache.insert_default_ttl("https://c.com", vec![3], HashMap::new());
        assert!(!cache.entries.contains_key("https://a.com"));
        assert!(cache.entries.contains_key("https://b.com"));
        assert!(cache.entries.contains_key("https://c.com"));
    }

    #[test]
    fn get_refreshes_lru() {
        let mut cache = TtlCache::new(2, Duration::from_secs(60));
        cache.insert_default_ttl("https://a.com", vec![1], HashMap::new());
        cache.insert_default_ttl("https://b.com", vec![2], HashMap::new());
        cache.get("https://a.com").unwrap();
        cache.insert_default_ttl("https://c.com", vec![3], HashMap::new());
        assert!(cache.entries.contains_key("https://a.com"));
        assert!(!cache.entries.contains_key("https://b.com"));
    }

    #[test]
    fn purge_expired_cleans() {
        let mut cache = TtlCache::new(10, Duration::from_secs(1));
        cache.insert("https://a.com".to_string(), vec![1], HashMap::new(), Duration::from_millis(1));
        cache.insert("https://b.com".to_string(), vec![2], HashMap::new(), Duration::from_secs(60));
        std::thread::sleep(Duration::from_millis(5));
        cache.purge_expired();
        assert!(!cache.entries.contains_key("https://a.com"));
        assert!(cache.entries.contains_key("https://b.com"));
    }

    #[test]
    fn clear_removes_all() {
        let mut cache = TtlCache::new(10, Duration::from_secs(60));
        cache.insert_default_ttl("https://a.com", vec![1], HashMap::new());
        cache.insert_default_ttl("https://b.com", vec![2], HashMap::new());
        cache.clear();
        assert_eq!(cache.len(), 0);
        assert!(cache.is_empty());
    }
}
