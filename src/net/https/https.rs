use crate::net::http::HttpClient;
use crate::net::fetch::Response;

pub struct HttpsClient {
    inner: HttpClient,
}

impl HttpsClient {
    pub fn new() -> Self {
        HttpsClient { inner: HttpClient::new() }
    }

    pub fn get(&self, url: &str) -> Result<Response, String> {
        self.inner.get(url)
    }
}
