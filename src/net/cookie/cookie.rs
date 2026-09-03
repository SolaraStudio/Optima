use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Cookie {
    pub name: String,
    pub value: String,
    pub domain: Option<String>,
    pub path: Option<String>,
    pub expires: Option<u64>,
    pub secure: bool,
    pub http_only: bool,
    pub same_site: Option<String>,
}

impl Cookie {
    pub fn new(name: &str, value: &str) -> Self {
        Cookie {
            name: name.to_string(),
            value: value.to_string(),
            domain: None,
            path: None,
            expires: None,
            secure: false,
            http_only: false,
            same_site: None,
        }
    }

    pub fn parse(header: &str) -> Vec<Cookie> {
        header
            .split(';')
            .filter_map(|pair| {
                let mut parts = pair.trim().splitn(2, '=');
                let name = parts.next()?.trim().to_string();
                let value = parts.next().unwrap_or("").trim().to_string();
                Some(Cookie::new(&name, &value))
            })
            .collect()
    }

    pub fn to_header(&self) -> String {
        format!("{}={}", self.name, self.value)
    }
}

pub struct CookieJar {
    cookies: HashMap<String, Cookie>,
}

impl CookieJar {
    pub fn new() -> Self {
        CookieJar {
            cookies: HashMap::new(),
        }
    }

    pub fn set(&mut self, cookie: Cookie) {
        self.cookies.insert(cookie.name.clone(), cookie);
    }

    pub fn get(&self, name: &str) -> Option<&Cookie> {
        self.cookies.get(name)
    }
    pub fn remove(&mut self, name: &str) {
        self.cookies.remove(name);
    }
    pub fn clear(&mut self) {
        self.cookies.clear();
    }

    pub fn to_header(&self) -> String {
        self.cookies
            .values()
            .map(|c| c.to_header())
            .collect::<Vec<_>>()
            .join("; ")
    }
}
