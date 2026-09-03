use crate::net::header::Headers;
use crate::net::body::Body;

#[derive(Debug, Clone, PartialEq)]
pub enum Method {
    GET,
    POST,
    PUT,
    DELETE,
    PATCH,
    HEAD,
    OPTIONS,
}

impl Method {
    pub fn as_str(&self) -> &str {
        match self {
            Method::GET => "GET", Method::POST => "POST", Method::PUT => "PUT",
            Method::DELETE => "DELETE", Method::PATCH => "PATCH",
            Method::HEAD => "HEAD", Method::OPTIONS => "OPTIONS",
        }
    }
}

#[derive(Debug, Clone)]
pub struct Request {
    pub url: String,
    pub method: Method,
    pub headers: Headers,
    pub body: Option<Body>,
}

impl Request {
    pub fn new(url: &str, method: Method) -> Self {
        Request { url: url.to_string(), method, headers: Headers::new(), body: None }
    }
    pub fn get(url: &str) -> Self { Self::new(url, Method::GET) }
    pub fn post(url: &str) -> Self { Self::new(url, Method::POST) }
    pub fn with_header(mut self, name: &str, value: &str) -> Self { self.headers.set(name, value); self }
    pub fn with_body(mut self, body: Body) -> Self { self.body = Some(body); self }
}

#[derive(Debug, Clone)]
pub struct Response {
    pub status: u16,
    pub status_text: String,
    pub headers: Headers,
    pub body: Body,
}

impl Response {
    pub fn ok() -> Self {
        Response { status: 200, status_text: "OK".to_string(), headers: Headers::new(), body: Body::new() }
    }
    pub fn error(status: u16, msg: &str) -> Self {
        Response { status, status_text: msg.to_string(), headers: Headers::new(), body: Body::new() }
    }
    pub fn is_ok(&self) -> bool { self.status >= 200 && self.status < 300 }
    pub fn text(&self) -> String { self.body.text() }
}

pub struct Fetch;

impl Fetch {
    pub fn execute(request: &Request) -> Result<Response, String> {
        Ok(Response::ok())
    }

    pub fn blocking_get(url: &str) -> Result<Response, String> {
        let req = Request::get(url);
        Self::execute(&req)
    }
}
