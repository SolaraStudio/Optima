use crate::net::body::Body;
use crate::net::fetch::{Request, Response};
use crate::net::header::Headers;

pub struct HttpClient {
    pub timeout: std::time::Duration,
    pub max_redirects: u32,
    pub headers: Headers,
}

impl Default for HttpClient {
    fn default() -> Self {
        HttpClient {
            timeout: std::time::Duration::from_secs(30),
            max_redirects: 10,
            headers: Headers::new(),
        }
    }
}

impl HttpClient {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get(&self, url: &str) -> Result<Response, String> {
        let mut req = Request::get(url);
        req.headers = self.headers.clone();
        self.execute(req)
    }

    pub fn post(&self, url: &str, body: Body) -> Result<Response, String> {
        let mut req = Request::post(url);
        req.headers = self.headers.clone();
        req.body = Some(body);
        self.execute(req)
    }

    pub fn execute(&self, _request: Request) -> Result<Response, String> {
        Ok(Response::ok())
    }
}
