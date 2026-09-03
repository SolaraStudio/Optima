use serde_json::Value;
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone)]
pub struct NetworkRequest {
    pub request_id: String,
    pub url: String,
    pub method: String,
    pub status: u16,
    pub status_text: String,
    pub request_time: u64,
    pub response_time: u64,
    pub duration: u64,
    pub request_headers: HashMap<String, String>,
    pub response_headers: HashMap<String, String>,
    pub request_body: Option<String>,
    pub response_body: Option<String>,
    pub mime_type: Option<String>,
    pub encoded_data_length: u64,
    pub decoded_body_length: u64,
}

pub struct NetworkBackend {
    pub requests: HashMap<String, NetworkRequest>,
    pub enabled: bool,
    pub max_requests: usize,
}

impl NetworkBackend {
    pub fn new() -> Self {
        NetworkBackend {
            requests: HashMap::new(),
            enabled: true,
            max_requests: 100,
        }
    }

    pub fn enable(&mut self) {
        self.enabled = true;
    }

    pub fn disable(&mut self) {
        self.enabled = false;
    }

    pub fn clear(&mut self) {
        self.requests.clear();
    }

    pub fn add_request(&mut self, request: NetworkRequest) {
        if !self.enabled {
            return;
        }
        if self.requests.len() >= self.max_requests {
            // Remove oldest request
            if let Some(oldest) = self.requests.keys().next().cloned() {
                self.requests.remove(&oldest);
            }
        }
        self.requests.insert(request.request_id.clone(), request);
    }

    pub fn get_request(&self, request_id: &str) -> Option<&NetworkRequest> {
        self.requests.get(request_id)
    }

    pub fn get_requests(&self) -> Vec<&NetworkRequest> {
        self.requests.values().collect()
    }

    pub fn to_json(&self) -> Value {
        let requests: Vec<Value> = self
            .requests
            .values()
            .map(|r| {
                serde_json::json!({
                    "requestId": r.request_id,
                    "url": r.url,
                    "method": r.method,
                    "status": r.status,
                    "statusText": r.status_text,
                    "duration": r.duration,
                    "requestHeaders": r.request_headers,
                    "responseHeaders": r.response_headers,
                    "mimeType": r.mime_type,
                    "encodedDataLength": r.encoded_data_length,
                    "decodedBodyLength": r.decoded_body_length
                })
            })
            .collect();
        serde_json::json!({ "requests": requests })
    }

    pub fn create_request(&mut self, url: &str, method: &str) -> String {
        let request_id = format!(
            "req-{}",
            std::time::SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_millis()
        );
        let request = NetworkRequest {
            request_id: request_id.clone(),
            url: url.to_string(),
            method: method.to_string(),
            status: 0,
            status_text: "".to_string(),
            request_time: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_millis() as u64,
            response_time: 0,
            duration: 0,
            request_headers: HashMap::new(),
            response_headers: HashMap::new(),
            request_body: None,
            response_body: None,
            mime_type: None,
            encoded_data_length: 0,
            decoded_body_length: 0,
        };
        self.add_request(request);
        request_id
    }

    pub fn update_request(&mut self, request_id: &str, status: u16, status_text: &str) {
        if let Some(request) = self.requests.get_mut(request_id) {
            request.status = status;
            request.status_text = status_text.to_string();
            request.response_time = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_millis() as u64;
            request.duration = request.response_time - request.request_time;
        }
    }
}

impl Default for NetworkBackend {
    fn default() -> Self {
        Self::new()
    }
}
