use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct MemorySnapshot {
    pub timestamp: u64,
    pub total_heap_size: u64,
    pub used_heap_size: u64,
    pub heap_size_limit: u64,
    pub total_physical_size: u64,
    pub total_available_size: u64,
    pub allocated_objects: usize,
    pub collections: u32,
    pub details: HashMap<String, u64>,
}

pub struct MemoryBackend {
    pub snapshots: Vec<MemorySnapshot>,
    pub enabled: bool,
    pub max_snapshots: usize,
}

impl MemoryBackend {
    pub fn new() -> Self {
        MemoryBackend {
            snapshots: Vec::new(),
            enabled: true,
            max_snapshots: 100,
        }
    }

    pub fn enable(&mut self) {
        self.enabled = true;
    }

    pub fn disable(&mut self) {
        self.enabled = false;
    }

    pub fn clear(&mut self) {
        self.snapshots.clear();
    }

    pub fn take_snapshot(&mut self) -> MemorySnapshot {
        let snapshot = MemorySnapshot {
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_millis() as u64,
            total_heap_size: 1024 * 1024 * 10,
            used_heap_size: 1024 * 1024 * 5,
            heap_size_limit: 1024 * 1024 * 100,
            total_physical_size: 1024 * 1024 * 20,
            total_available_size: 1024 * 1024 * 30,
            allocated_objects: 1000,
            collections: 5,
            details: HashMap::new(),
        };
        if self.enabled {
            self.snapshots.push(snapshot.clone());
            if self.snapshots.len() > self.max_snapshots {
                self.snapshots.remove(0);
            }
        }
        snapshot
    }

    pub fn get_snapshots(&self) -> &[MemorySnapshot] {
        &self.snapshots
    }

    pub fn get_latest_snapshot(&self) -> Option<&MemorySnapshot> {
        self.snapshots.last()
    }

    pub fn to_json(&self) -> Value {
        let snapshots: Vec<Value> = self.snapshots.iter().map(|s| {
            serde_json::json!({
                "timestamp": s.timestamp,
                "totalHeapSize": s.total_heap_size,
                "usedHeapSize": s.used_heap_size,
                "heapSizeLimit": s.heap_size_limit,
                "totalPhysicalSize": s.total_physical_size,
                "totalAvailableSize": s.total_available_size,
                "allocatedObjects": s.allocated_objects,
                "collections": s.collections,
                "details": s.details
            })
        }).collect();
        serde_json::json!({ "snapshots": snapshots })
    }

    pub fn get_dom_counters(&self) -> Value {
        serde_json::json!({
            "documents": 1,
            "nodes": 10,
            "jsEventListeners": 0
        })
    }
}

impl Default for MemoryBackend {
    fn default() -> Self {
        Self::new()
    }
}
