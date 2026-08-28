use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT};
use std::time::Duration;

pub struct HttpClient {
    client: Client,
    user_agent: String,
}

impl HttpClient {
    pub fn new() -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .unwrap();
        Self {
            client,
            user_agent: "Optima/0.150.10-dev".to_string(),
        }
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

    pub fn post(&self, url: &str, body: &[u8]) -> Result<String, reqwest::Error> {
        let resp = self.client.post(url).body(body.to_vec()).send()?;
        resp.text()
    }
}
