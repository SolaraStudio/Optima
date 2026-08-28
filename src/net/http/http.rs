use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT, CONTENT_TYPE};
use std::time::Duration;

pub struct HttpClient {
    client: Client,
    user_agent: String,
    timeout: Duration,
}

impl HttpClient {
    pub fn new() -> Self {
        let timeout = Duration::from_secs(30);
        let client = Client::builder()
            .timeout(timeout)
            .build()
            .unwrap();
        Self {
            client,
            user_agent: "Optima/0.150.10-dev".to_string(),
            timeout,
        }
    }

    pub fn with_user_agent(mut self, ua: &str) -> Self {
        self.user_agent = ua.to_string();
        self
    }

    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self.client = Client::builder()
            .timeout(timeout)
            .build()
            .unwrap();
        self
    }

    pub fn get(&self, url: &str) -> Result<String, reqwest::Error> {
        let mut headers = HeaderMap::new();
        headers.insert(USER_AGENT, HeaderValue::from_str(&self.user_agent).unwrap());
        let resp = self.client.get(url).headers(headers).send()?;
        resp.text()
    }

    pub fn get_bytes(&self, url: &str) -> Result<Vec<u8>, reqwest::Error> {
        let mut headers = HeaderMap::new();
        headers.insert(USER_AGENT, HeaderValue::from_str(&self.user_agent).unwrap());
        let resp = self.client.get(url).headers(headers).send()?;
        resp.bytes().map(|b| b.to_vec())
    }

    pub fn get_json<T: serde::de::DeserializeOwned>(&self, url: &str) -> Result<T, reqwest::Error> {
        let mut headers = HeaderMap::new();
        headers.insert(USER_AGENT, HeaderValue::from_str(&self.user_agent).unwrap());
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        let resp = self.client.get(url).headers(headers).send()?;
        resp.json()
    }

    pub fn post(&self, url: &str, body: &[u8]) -> Result<String, reqwest::Error> {
        let mut headers = HeaderMap::new();
        headers.insert(USER_AGENT, HeaderValue::from_str(&self.user_agent).unwrap());
        let resp = self.client.post(url).body(body.to_vec()).headers(headers).send()?;
        resp.text()
    }

    pub fn post_json<T: serde::Serialize>(&self, url: &str, data: &T) -> Result<String, reqwest::Error> {
        let mut headers = HeaderMap::new();
        headers.insert(USER_AGENT, HeaderValue::from_str(&self.user_agent).unwrap());
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        let resp = self.client.post(url).json(data).headers(headers).send()?;
        resp.text()
    }

    pub fn put(&self, url: &str, body: &[u8]) -> Result<String, reqwest::Error> {
        let mut headers = HeaderMap::new();
        headers.insert(USER_AGENT, HeaderValue::from_str(&self.user_agent).unwrap());
        let resp = self.client.put(url).body(body.to_vec()).headers(headers).send()?;
        resp.text()
    }

    pub fn delete(&self, url: &str) -> Result<String, reqwest::Error> {
        let mut headers = HeaderMap::new();
        headers.insert(USER_AGENT, HeaderValue::from_str(&self.user_agent).unwrap());
        let resp = self.client.delete(url).headers(headers).send()?;
        resp.text()
    }

    pub fn head(&self, url: &str) -> Result<reqwest::blocking::Response, reqwest::Error> {
        let mut headers = HeaderMap::new();
        headers.insert(USER_AGENT, HeaderValue::from_str(&self.user_agent).unwrap());
        self.client.head(url).headers(headers).send()
    }

    pub fn options(&self, url: &str) -> Result<reqwest::blocking::Response, reqwest::Error> {
        let mut headers = HeaderMap::new();
        headers.insert(USER_AGENT, HeaderValue::from_str(&self.user_agent).unwrap());
        self.client.request(reqwest::Method::OPTIONS, url).headers(headers).send()
    }

    pub fn get_response(&self, url: &str) -> Result<reqwest::blocking::Response, reqwest::Error> {
        let mut headers = HeaderMap::new();
        headers.insert(USER_AGENT, HeaderValue::from_str(&self.user_agent).unwrap());
        self.client.get(url).headers(headers).send()
    }

    pub fn get_user_agent(&self) -> &str {
        &self.user_agent
    }

    pub fn get_timeout(&self) -> Duration {
        self.timeout
    }
}

impl Default for HttpClient {
    fn default() -> Self {
        Self::new()
    }
}
