use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[derive(Default)]
pub enum HttpMethod {
    #[default]
    Get,
    Post,
    Put,
    Delete,
    Patch,
    Head,
    Options,
}


impl HttpMethod {
    pub fn as_str(&self) -> &'static str {
        match self {
            HttpMethod::Get => "GET",
            HttpMethod::Post => "POST",
            HttpMethod::Put => "PUT",
            HttpMethod::Delete => "DELETE",
            HttpMethod::Patch => "PATCH",
            HttpMethod::Head => "HEAD",
            HttpMethod::Options => "OPTIONS",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[derive(Default)]
pub enum RequestStage {
    #[default]
    Pending,
    Sending,
    Waiting,
    Receiving,
    Complete,
    Failed,
    Cancelled,
}


#[derive(Debug, Clone)]
pub struct HttpRequest {
    pub url: String,
    pub method: HttpMethod,
    pub headers: HashMap<String, String>,
    pub body: Option<Vec<u8>>,
    pub timestamp_ms: f64,
}

impl HttpRequest {
    pub fn new(method: HttpMethod, url: &str) -> Self {
        HttpRequest {
            url: url.to_string(),
            method,
            headers: HashMap::new(),
            body: None,
            timestamp_ms: 0.0,
        }
    }

    pub fn with_header(mut self, key: &str, value: &str) -> Self {
        self.headers.insert(key.to_string(), value.to_string());
        self
    }

    pub fn with_body(mut self, body: Vec<u8>) -> Self {
        self.body = Some(body);
        self
    }

    pub fn with_timestamp(mut self, ms: f64) -> Self {
        self.timestamp_ms = ms;
        self
    }

    pub fn content_length(&self) -> usize {
        self.body.as_ref().map_or(0, |b| b.len())
    }

    pub fn header(&self, key: &str) -> Option<&String> {
        self.headers.get(key)
    }
}

#[derive(Debug, Clone)]
pub struct HttpResponse {
    pub status: u16,
    pub status_text: String,
    pub headers: HashMap<String, String>,
    pub body: Option<Vec<u8>>,
    pub content_type: Option<String>,
}

impl Default for HttpResponse {
    fn default() -> Self {
        HttpResponse {
            status: 200,
            status_text: "OK".to_string(),
            headers: HashMap::new(),
            body: None,
            content_type: None,
        }
    }
}

impl HttpResponse {
    pub fn new(status: u16) -> Self {
        let status_text = match status {
            200 => "OK",
            201 => "Created",
            204 => "No Content",
            301 => "Moved Permanently",
            304 => "Not Modified",
            400 => "Bad Request",
            401 => "Unauthorized",
            403 => "Forbidden",
            404 => "Not Found",
            500 => "Internal Server Error",
            502 => "Bad Gateway",
            503 => "Service Unavailable",
            _ => "Unknown",
        };
        HttpResponse {
            status,
            status_text: status_text.to_string(),
            headers: HashMap::new(),
            body: None,
            content_type: None,
        }
    }

    pub fn with_header(mut self, key: &str, value: &str) -> Self {
        if key.eq_ignore_ascii_case("content-type") {
            self.content_type = Some(value.to_string());
        }
        self.headers.insert(key.to_string(), value.to_string());
        self
    }

    pub fn with_body(mut self, body: Vec<u8>) -> Self {
        self.body = Some(body);
        self
    }

    pub fn is_success(&self) -> bool {
        self.status >= 200 && self.status < 300
    }

    pub fn is_redirect(&self) -> bool {
        self.status >= 300 && self.status < 400
    }

    pub fn is_client_error(&self) -> bool {
        self.status >= 400 && self.status < 500
    }

    pub fn is_server_error(&self) -> bool {
        self.status >= 500
    }

    pub fn body_size(&self) -> usize {
        self.body.as_ref().map_or(0, |b| b.len())
    }
}

#[derive(Debug, Clone)]
pub struct RequestTiming {
    pub start_ms: f64,
    pub dns_ms: Option<f64>,
    pub connect_ms: Option<f64>,
    pub tls_ms: Option<f64>,
    pub request_sent_ms: Option<f64>,
    pub first_byte_ms: Option<f64>,
    pub end_ms: Option<f64>,
}

impl Default for RequestTiming {
    fn default() -> Self {
        RequestTiming {
            start_ms: 0.0,
            dns_ms: None,
            connect_ms: None,
            tls_ms: None,
            request_sent_ms: None,
            first_byte_ms: None,
            end_ms: None,
        }
    }
}

impl RequestTiming {
    pub fn total_ms(&self) -> Option<f64> {
        self.end_ms.map(|end| end - self.start_ms)
    }

    pub fn dns_duration_ms(&self) -> Option<f64> {
        self.dns_ms.map(|dns| dns - self.start_ms)
    }

    pub fn connect_duration_ms(&self) -> Option<f64> {
        self.connect_ms.map(|c| c - self.start_ms)
    }

    pub fn wait_time_ms(&self) -> Option<f64> {
        match (self.first_byte_ms, self.request_sent_ms) {
            (Some(fb), Some(rs)) => Some(fb - rs),
            _ => None,
        }
    }

    pub fn download_time_ms(&self) -> Option<f64> {
        match (self.end_ms, self.first_byte_ms) {
            (Some(end), Some(fb)) => Some(end - fb),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct NetworkRequest {
    pub id: u32,
    pub request: HttpRequest,
    pub response: Option<HttpResponse>,
    pub timing: RequestTiming,
    pub stage: RequestStage,
    pub error: Option<String>,
    pub response_size: usize,
}

impl NetworkRequest {
    pub fn new(id: u32, request: HttpRequest) -> Self {
        NetworkRequest {
            id,
            request,
            response: None,
            timing: RequestTiming::default(),
            stage: RequestStage::Pending,
            error: None,
            response_size: 0,
        }
    }

    pub fn start(&mut self, time_ms: f64) {
        self.stage = RequestStage::Sending;
        self.timing.start_ms = time_ms;
    }

    pub fn wait(&mut self) {
        self.stage = RequestStage::Waiting;
    }

    pub fn receive(&mut self, response: HttpResponse, time_ms: f64) {
        self.response = Some(response);
        self.timing.first_byte_ms = Some(time_ms);
        self.stage = RequestStage::Receiving;
    }

    pub fn complete(&mut self, time_ms: f64) {
        self.timing.end_ms = Some(time_ms);
        self.stage = RequestStage::Complete;
        if let Some(ref resp) = self.response {
            self.response_size = resp.body_size();
        }
    }

    pub fn fail(&mut self, error: &str, time_ms: f64) {
        self.error = Some(error.to_string());
        self.timing.end_ms = Some(time_ms);
        self.stage = RequestStage::Failed;
    }

    pub fn cancel(&mut self) {
        self.stage = RequestStage::Cancelled;
    }

    pub fn duration_ms(&self) -> f64 {
        self.timing.total_ms().unwrap_or(0.0)
    }

    pub fn is_complete(&self) -> bool {
        self.stage == RequestStage::Complete || self.stage == RequestStage::Failed
    }

    pub fn status_code(&self) -> Option<u16> {
        self.response.as_ref().map(|r| r.status)
    }

    pub fn url_path(&self) -> &str {
        self.request
            .url
            .find('/')
            .map_or(&self.request.url, |i| &self.request.url[i..])
    }
}

#[derive(Debug)]
pub struct NetworkInspector {
    pub requests: Vec<NetworkRequest>,
    pub max_requests: usize,
    pub next_id: u32,
    pub total_requests: u64,
    pub total_failed: u64,
    pub total_bytes_received: u64,
    pub base_url: Option<String>,
    pub blocked_urls: Vec<String>,
    pub latency_overrides: HashMap<String, f64>,
}

impl Default for NetworkInspector {
    fn default() -> Self {
        NetworkInspector {
            requests: Vec::new(),
            max_requests: 500,
            next_id: 0,
            total_requests: 0,
            total_failed: 0,
            total_bytes_received: 0,
            base_url: None,
            blocked_urls: Vec::new(),
            latency_overrides: HashMap::new(),
        }
    }
}

impl NetworkInspector {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_max_requests(mut self, max: usize) -> Self {
        self.max_requests = max;
        self
    }

    pub fn with_base_url(mut self, url: &str) -> Self {
        self.base_url = Some(url.to_string());
        self
    }

    pub fn add_request(&mut self, request: HttpRequest) -> u32 {
        let id = self.next_id;
        self.next_id += 1;
        self.total_requests += 1;

        let mut net_req = NetworkRequest::new(id, request);
        net_req.start(net_req.request.timestamp_ms);
        self.requests.push(net_req);

        if self.requests.len() > self.max_requests {
            self.requests.remove(0);
        }

        id
    }

    pub fn get_request(&self, id: u32) -> Option<&NetworkRequest> {
        self.requests.iter().find(|r| r.id == id)
    }

    pub fn get_request_mut(&mut self, id: u32) -> Option<&mut NetworkRequest> {
        self.requests.iter_mut().find(|r| r.id == id)
    }

    pub fn complete_request(&mut self, id: u32, response: HttpResponse, time_ms: f64) {
        if let Some(req) = self.get_request_mut(id) {
            req.receive(response, time_ms);
            req.complete(time_ms);
            self.total_bytes_received += req.response_size as u64;
        }
    }

    pub fn fail_request(&mut self, id: u32, error: &str, time_ms: f64) {
        if let Some(req) = self.get_request_mut(id) {
            req.fail(error, time_ms);
            self.total_failed += 1;
        }
    }

    pub fn cancel_request(&mut self, id: u32) {
        if let Some(req) = self.get_request_mut(id) {
            req.cancel();
        }
    }

    pub fn block_url(&mut self, pattern: &str) {
        if !self.blocked_urls.contains(&pattern.to_string()) {
            self.blocked_urls.push(pattern.to_string());
        }
    }

    pub fn is_url_blocked(&self, url: &str) -> bool {
        self.blocked_urls.iter().any(|p| url.contains(p.as_str()))
    }

    pub fn set_latency_override(&mut self, url_pattern: &str, latency_ms: f64) {
        self.latency_overrides
            .insert(url_pattern.to_string(), latency_ms);
    }

    pub fn get_latency_override(&self, url: &str) -> Option<f64> {
        self.latency_overrides
            .iter()
            .find(|(k, _)| url.contains(k.as_str()))
            .map(|(_, v)| *v)
    }

    pub fn active_requests(&self) -> Vec<&NetworkRequest> {
        self.requests.iter().filter(|r| !r.is_complete()).collect()
    }

    pub fn failed_requests(&self) -> Vec<&NetworkRequest> {
        self.requests
            .iter()
            .filter(|r| r.stage == RequestStage::Failed)
            .collect()
    }

    pub fn requests_by_url(&self, url: &str) -> Vec<&NetworkRequest> {
        self.requests
            .iter()
            .filter(|r| r.request.url.contains(url))
            .collect()
    }

    pub fn requests_by_status(&self, status: u16) -> Vec<&NetworkRequest> {
        self.requests
            .iter()
            .filter(|r| r.status_code() == Some(status))
            .collect()
    }

    pub fn slowest_requests(&self, count: usize) -> Vec<&NetworkRequest> {
        let mut completed: Vec<&NetworkRequest> =
            self.requests.iter().filter(|r| r.is_complete()).collect();
        completed.sort_by(|a, b| {
            b.duration_ms()
                .partial_cmp(&a.duration_ms())
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        completed.into_iter().take(count).collect()
    }

    pub fn clear(&mut self) {
        self.requests.clear();
        self.total_requests = 0;
        self.total_failed = 0;
        self.total_bytes_received = 0;
    }

    pub fn summary(&self) -> NetworkSummary {
        let completed: Vec<&NetworkRequest> =
            self.requests.iter().filter(|r| r.is_complete()).collect();
        let total_duration: f64 = completed.iter().map(|r| r.duration_ms()).sum();
        let avg_duration = if completed.is_empty() {
            0.0
        } else {
            total_duration / completed.len() as f64
        };

        NetworkSummary {
            total_requests: self.total_requests,
            active: self.active_requests().len(),
            failed: self.total_failed,
            total_bytes: self.total_bytes_received,
            avg_duration_ms: avg_duration,
        }
    }
}

#[derive(Debug, Clone)]
pub struct NetworkSummary {
    pub total_requests: u64,
    pub active: usize,
    pub failed: u64,
    pub total_bytes: u64,
    pub avg_duration_ms: f64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_http_method() {
        assert_eq!(HttpMethod::Get.as_str(), "GET");
        assert_eq!(HttpMethod::Post.as_str(), "POST");
        assert_eq!(HttpMethod::Delete.as_str(), "DELETE");
    }

    #[test]
    fn test_http_request() {
        let req = HttpRequest::new(HttpMethod::Get, "https://example.com/api")
            .with_header("Authorization", "Bearer token")
            .with_body(vec![1, 2, 3])
            .with_timestamp(1000.0);

        assert_eq!(req.method, HttpMethod::Get);
        assert_eq!(req.url, "https://example.com/api");
        assert_eq!(req.content_length(), 3);
        assert_eq!(
            req.header("Authorization"),
            Some(&"Bearer token".to_string())
        );
    }

    #[test]
    fn test_http_response() {
        let resp = HttpResponse::new(200)
            .with_header("Content-Type", "application/json")
            .with_body(b"{}".to_vec());

        assert!(resp.is_success());
        assert!(!resp.is_redirect());
        assert!(!resp.is_client_error());
        assert_eq!(resp.body_size(), 2);
        assert_eq!(resp.content_type, Some("application/json".to_string()));
    }

    #[test]
    fn test_http_response_codes() {
        assert!(HttpResponse::new(301).is_redirect());
        assert!(HttpResponse::new(404).is_client_error());
        assert!(HttpResponse::new(500).is_server_error());
    }

    #[test]
    fn test_request_timing() {
        let timing = RequestTiming {
            start_ms: 100.0,
            dns_ms: Some(105.0),
            connect_ms: Some(110.0),
            request_sent_ms: Some(112.0),
            first_byte_ms: Some(130.0),
            end_ms: Some(150.0),
            ..Default::default()
        };
        assert_eq!(timing.total_ms(), Some(50.0));
        assert_eq!(timing.dns_duration_ms(), Some(5.0));
        assert_eq!(timing.wait_time_ms(), Some(18.0));
        assert_eq!(timing.download_time_ms(), Some(20.0));

        let empty = RequestTiming::default();
        assert_eq!(empty.total_ms(), None);
    }

    #[test]
    fn test_network_request() {
        let req = HttpRequest::new(HttpMethod::Get, "https://api.test.com/data");
        let mut net = NetworkRequest::new(1, req);

        net.start(100.0);
        assert_eq!(net.stage, RequestStage::Sending);

        net.wait();
        assert_eq!(net.stage, RequestStage::Waiting);

        let resp = HttpResponse::new(200);
        net.receive(resp, 120.0);
        assert_eq!(net.stage, RequestStage::Receiving);

        net.complete(130.0);
        assert!(net.is_complete());
        assert_eq!(net.duration_ms(), 30.0);
    }

    #[test]
    fn test_network_request_fail() {
        let req = HttpRequest::new(HttpMethod::Get, "https://fail.test.com");
        let mut net = NetworkRequest::new(1, req);
        net.start(100.0);
        net.fail("Connection refused", 110.0);
        assert!(net.is_complete());
        assert_eq!(net.error, Some("Connection refused".to_string()));
    }

    #[test]
    fn test_network_inspector() {
        let mut inspector = NetworkInspector::new();
        let req = HttpRequest::new(HttpMethod::Get, "https://api.test.com/data")
            .with_header("Accept", "application/json");
        let id = inspector.add_request(req);

        assert_eq!(inspector.total_requests, 1);
        assert_eq!(inspector.active_requests().len(), 1);

        let resp = HttpResponse::new(200).with_body(b"ok".to_vec());
        inspector.complete_request(id, resp, 150.0);
        assert_eq!(inspector.active_requests().len(), 0);
        assert_eq!(inspector.total_bytes_received, 2);
    }

    #[test]
    fn test_inspector_filtering() {
        let mut inspector = NetworkInspector::new();

        let id1 = inspector.add_request(HttpRequest::new(HttpMethod::Get, "https://a.com/1"));
        inspector.complete_request(id1, HttpResponse::new(200), 100.0);

        let id2 = inspector.add_request(HttpRequest::new(HttpMethod::Post, "https://b.com/2"));
        inspector.fail_request(id2, "timeout", 200.0);

        let id3 = inspector.add_request(HttpRequest::new(HttpMethod::Get, "https://a.com/3"));
        inspector.complete_request(id3, HttpResponse::new(404), 150.0);

        assert_eq!(inspector.requests_by_url("a.com").len(), 2);
        assert_eq!(inspector.requests_by_status(200).len(), 1);
        assert_eq!(inspector.failed_requests().len(), 1);
    }

    #[test]
    fn test_inspector_slowest() {
        let mut inspector = NetworkInspector::new();
        let id1 = inspector.add_request(HttpRequest::new(HttpMethod::Get, "fast"));
        inspector.complete_request(id1, HttpResponse::new(200), 10.0);

        let id2 = inspector.add_request(HttpRequest::new(HttpMethod::Get, "slow"));
        inspector.complete_request(id2, HttpResponse::new(200), 200.0);

        let slowest = inspector.slowest_requests(1);
        assert_eq!(slowest[0].request.url, "slow");
    }

    #[test]
    fn test_block_urls() {
        let mut inspector = NetworkInspector::new();
        inspector.block_url("ads.example.com");
        assert!(inspector.is_url_blocked("https://ads.example.com/banner"));
        assert!(!inspector.is_url_blocked("https://api.example.com/data"));
    }

    #[test]
    fn test_latency_override() {
        let mut inspector = NetworkInspector::new();
        inspector.set_latency_override("api.test.com", 500.0);
        assert_eq!(
            inspector.get_latency_override("https://api.test.com/data"),
            Some(500.0)
        );
        assert_eq!(inspector.get_latency_override("https://other.com"), None);
    }

    #[test]
    fn test_inspector_summary() {
        let mut inspector = NetworkInspector::new();
        let id1 = inspector.add_request(HttpRequest::new(HttpMethod::Get, "a"));
        inspector.complete_request(id1, HttpResponse::new(200), 100.0);
        let id2 = inspector.add_request(HttpRequest::new(HttpMethod::Get, "b"));
        inspector.complete_request(id2, HttpResponse::new(500), 200.0);

        let summary = inspector.summary();
        assert_eq!(summary.total_requests, 2);
        assert_eq!(summary.active, 0);
        assert!((summary.avg_duration_ms - 150.0).abs() < 0.01);
    }

    #[test]
    fn test_inspector_clear() {
        let mut inspector = NetworkInspector::new();
        inspector.add_request(HttpRequest::new(HttpMethod::Get, "test"));
        inspector.clear();
        assert_eq!(inspector.total_requests, 0);
        assert!(inspector.requests.is_empty());
    }

    #[test]
    fn test_inspector_cancel() {
        let mut inspector = NetworkInspector::new();
        let id = inspector.add_request(HttpRequest::new(HttpMethod::Get, "test"));
        inspector.cancel_request(id);
        let req = inspector.get_request(id).unwrap();
        assert_eq!(req.stage, RequestStage::Cancelled);
    }
}
