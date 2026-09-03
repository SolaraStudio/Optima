use serde_json::Value;
use std::collections::VecDeque;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone)]
pub struct PerformanceMetric {
    pub name: String,
    pub value: f64,
    pub timestamp: u64,
    pub metadata: std::collections::HashMap<String, String>,
}

pub struct PerformanceBackend {
    pub metrics: VecDeque<PerformanceMetric>,
    pub enabled: bool,
    pub max_metrics: usize,
}

impl PerformanceBackend {
    pub fn new() -> Self {
        PerformanceBackend {
            metrics: VecDeque::new(),
            enabled: true,
            max_metrics: 1000,
        }
    }

    pub fn enable(&mut self) {
        self.enabled = true;
    }

    pub fn disable(&mut self) {
        self.enabled = false;
    }

    pub fn clear(&mut self) {
        self.metrics.clear();
    }

    pub fn add_metric(
        &mut self,
        name: &str,
        value: f64,
        metadata: std::collections::HashMap<String, String>,
    ) {
        if !self.enabled {
            return;
        }
        let metric = PerformanceMetric {
            name: name.to_string(),
            value,
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_millis() as u64,
            metadata,
        };
        self.metrics.push_back(metric);
        if self.metrics.len() > self.max_metrics {
            self.metrics.pop_front();
        }
    }

    pub fn get_metrics(&self) -> Vec<&PerformanceMetric> {
        self.metrics.iter().collect()
    }

    pub fn get_metrics_by_name(&self, name: &str) -> Vec<&PerformanceMetric> {
        self.metrics.iter().filter(|m| m.name == name).collect()
    }

    pub fn to_json(&self) -> Value {
        let metrics: Vec<Value> = self
            .metrics
            .iter()
            .map(|m| {
                serde_json::json!({
                    "name": m.name,
                    "value": m.value,
                    "timestamp": m.timestamp,
                    "metadata": m.metadata
                })
            })
            .collect();
        serde_json::json!({ "metrics": metrics })
    }
}

impl Default for PerformanceBackend {
    fn default() -> Self {
        Self::new()
    }
}
