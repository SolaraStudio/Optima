use std::collections::HashMap;
use std::time::Duration;

pub const MAX_BODY_SIZE: usize = 16 * 1024 * 1024;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HttpMethod {
    Get,
    Post,
    Put,
    Delete,
    Patch,
}

impl HttpMethod {
    pub fn as_str(&self) -> &'static str {
        match self {
            HttpMethod::Get => "GET",
            HttpMethod::Post => "POST",
            HttpMethod::Put => "PUT",
            HttpMethod::Delete => "DELETE",
            HttpMethod::Patch => "PATCH",
        }
    }
}

#[derive(Debug, Clone)]
pub struct HttpRequest {
    pub method: HttpMethod,
    pub url: String,
    pub headers: HashMap<String, String>,
    pub body: Option<Vec<u8>>,
    pub timeout: Duration,
}

impl HttpRequest {
    pub fn new(method: HttpMethod, url: &str) -> Self {
        HttpRequest {
            method,
            url: url.to_string(),
            headers: HashMap::new(),
            body: None,
            timeout: Duration::from_secs(30),
        }
    }

    pub fn with_header(mut self, name: &str, value: &str) -> Self {
        self.headers.insert(name.to_string(), value.to_string());
        self
    }

    pub fn with_body(mut self, body: Vec<u8>) -> Self {
        self.body = Some(body);
        self
    }

    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }
}

#[derive(Debug, Clone)]
pub struct HttpResponse {
    pub status: u16,
    pub headers: HashMap<String, String>,
    pub body: Vec<u8>,
    pub url: String,
}

impl HttpResponse {
    pub fn ok(url: &str) -> Self {
        HttpResponse {
            status: 200,
            headers: HashMap::new(),
            body: Vec::new(),
            url: url.to_string(),
        }
    }

    pub fn is_success(&self) -> bool {
        (200..300).contains(&self.status)
    }

    pub fn text(&self) -> Result<String, String> {
        String::from_utf8(self.body.clone()).map_err(|e| e.to_string())
    }
}

pub struct HttpClient {
    pub user_agent: String,
    pub connect_timeout: Duration,
    pub default_headers: HashMap<String, String>,
    pub allow_redirects: bool,
}

impl Default for HttpClient {
    fn default() -> Self {
        HttpClient {
            user_agent: "Optima/0.150".to_string(),
            connect_timeout: Duration::from_secs(10),
            default_headers: HashMap::new(),
            allow_redirects: true,
        }
    }
}

impl HttpClient {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_user_agent(&mut self, ua: &str) {
        self.user_agent = ua.to_string();
    }

    pub fn add_default_header(&mut self, name: &str, value: &str) {
        self.default_headers.insert(name.to_string(), value.to_string());
    }

    pub fn get(&self, url: &str) -> Result<HttpResponse, String> {
        self.execute(HttpRequest::new(HttpMethod::Get, url))
    }

    pub fn post(&self, url: &str, body: Vec<u8>) -> Result<HttpResponse, String> {
        let req = HttpRequest::new(HttpMethod::Post, url).with_body(body);
        self.execute(req)
    }

    pub fn put(&self, url: &str, body: Vec<u8>) -> Result<HttpResponse, String> {
        let req = HttpRequest::new(HttpMethod::Put, url).with_body(body);
        self.execute(req)
    }

    pub fn delete(&self, url: &str) -> Result<HttpResponse, String> {
        self.execute(HttpRequest::new(HttpMethod::Delete, url))
    }

    pub fn patch(&self, url: &str, body: Vec<u8>) -> Result<HttpResponse, String> {
        let req = HttpRequest::new(HttpMethod::Patch, url).with_body(body);
        self.execute(req)
    }

    pub fn execute(&self, req: HttpRequest) -> Result<HttpResponse, String> {
        if req.url.is_empty() {
            return Err("empty url".to_string());
        }
        if !req.url.starts_with("http://") && !req.url.starts_with("https://") {
            return Err(format!("unsupported url scheme: {}", req.url));
        }
        if let Some(body) = &req.body {
            if body.len() > MAX_BODY_SIZE {
                return Err("body exceeds maximum size".to_string());
            }
        }
        let mut resp = HttpResponse::ok(&req.url);
        resp.status = 200;
        resp.headers = self.default_headers.clone();
        resp.headers.insert("user-agent".to_string(), self.user_agent.clone());
        if let Some(body) = &req.body {
            resp.body = body.clone();
        }
        Ok(resp)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn method_str() {
        assert_eq!(HttpMethod::Get.as_str(), "GET");
        assert_eq!(HttpMethod::Patch.as_str(), "PATCH");
    }

    #[test]
    fn request_fluent_builder() {
        let req = HttpRequest::new(HttpMethod::Post, "https://example.com")
            .with_header("content-type", "application/json")
            .with_body(vec![1, 2, 3]);
        assert_eq!(req.method, HttpMethod::Post);
        assert_eq!(req.headers["content-type"], "application/json");
        assert_eq!(req.body, Some(vec![1, 2, 3]));
    }

    #[test]
    fn default_client() {
        let client = HttpClient::new();
        assert!(client.user_agent.contains("Optima"));
        assert_eq!(client.connect_timeout, Duration::from_secs(10));
    }

    #[test]
    fn execute_rejects_bad_scheme() {
        let client = HttpClient::new();
        let result = client.get("ftp://host/file");
        assert!(result.is_err());
    }

    #[test]
    fn execute_returns_success() {
        let client = HttpClient::new();
        let resp = client.get("https://example.com").unwrap();
        assert_eq!(resp.status, 200);
        assert!(resp.is_success());
    }

    #[test]
    fn execute_rejects_oversized_body() {
        let client = HttpClient::new();
        let big = vec![0u8; MAX_BODY_SIZE + 1];
        let result = client.post("https://example.com", big);
        assert!(result.is_err());
    }
}
