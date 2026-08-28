use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Cookie {
    pub name: String,
    pub value: String,
    pub domain: String,
    pub path: String,
    pub secure: bool,
    pub http_only: bool,
    pub max_age: Option<u64>,
}

pub struct CookieManager {
    cookies: HashMap<String, Vec<Cookie>>,
}

impl CookieManager {
    pub fn new() -> Self {
        Self {
            cookies: HashMap::new(),
        }
    }

    pub fn add_cookie(&mut self, domain: &str, cookie: Cookie) {
        let entry = self.cookies.entry(domain.to_string()).or_insert_with(Vec::new);
        entry.push(cookie);
    }

    pub fn get_cookies(&self, domain: &str) -> Vec<&Cookie> {
        if let Some(cookies) = self.cookies.get(domain) {
            cookies.iter().collect()
        } else {
            Vec::new()
        }
    }

    pub fn get_cookie_string(&self, domain: &str) -> String {
        let cookies = self.get_cookies(domain);
        cookies
            .iter()
            .map(|c| format!("{}={}", c.name, c.value))
            .collect::<Vec<String>>()
            .join("; ")
    }

    pub fn clear_domain(&mut self, domain: &str) {
        self.cookies.remove(domain);
    }

    pub fn clear_all(&mut self) {
        self.cookies.clear();
    }
}
