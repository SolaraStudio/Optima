use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Cookie {
    pub name: String,
    pub value: String,
    pub domain: String,
    pub path: String,
    pub secure: bool,
    pub http_only: bool,
    pub same_site: SameSite,
    pub max_age: Option<u64>,
    pub expires: Option<u64>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SameSite {
    Strict,
    Lax,
    None,
    Default,
}

impl Cookie {
    pub fn new(name: &str, value: &str, domain: &str) -> Self {
        Self {
            name: name.to_string(),
            value: value.to_string(),
            domain: domain.to_string(),
            path: "/".to_string(),
            secure: false,
            http_only: false,
            same_site: SameSite::Default,
            max_age: None,
            expires: None,
        }
    }

    pub fn with_path(mut self, path: &str) -> Self {
        self.path = path.to_string();
        self
    }

    pub fn with_secure(mut self, secure: bool) -> Self {
        self.secure = secure;
        self
    }

    pub fn with_http_only(mut self, http_only: bool) -> Self {
        self.http_only = http_only;
        self
    }

    pub fn with_same_site(mut self, same_site: SameSite) -> Self {
        self.same_site = same_site;
        self
    }

    pub fn with_max_age(mut self, max_age: u64) -> Self {
        self.max_age = Some(max_age);
        self
    }

    pub fn with_expires(mut self, expires: u64) -> Self {
        self.expires = Some(expires);
        self
    }

    pub fn to_string(&self) -> String {
        let mut s = format!("{}={}", self.name, self.value);
        if let Some(max_age) = self.max_age {
            s.push_str(&format!("; Max-Age={}", max_age));
        }
        if let Some(expires) = self.expires {
            s.push_str(&format!("; Expires={}", expires));
        }
        if self.secure {
            s.push_str("; Secure");
        }
        if self.http_only {
            s.push_str("; HttpOnly");
        }
        match self.same_site {
            SameSite::Strict => s.push_str("; SameSite=Strict"),
            SameSite::Lax => s.push_str("; SameSite=Lax"),
            SameSite::None => s.push_str("; SameSite=None"),
            SameSite::Default => {}
        }
        s
    }
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

    pub fn set_cookie(&mut self, domain: &str, cookie_str: &str) -> Option<Cookie> {
        let parts: Vec<&str> = cookie_str.split(';').map(|s| s.trim()).collect();
        let first_part = parts.first()?;
        let cookie_parts: Vec<&str> = first_part.split('=').collect();
        if cookie_parts.len() != 2 {
            return None;
        }
        let name = cookie_parts[0].to_string();
        let value = cookie_parts[1].to_string();
        let mut cookie = Cookie::new(&name, &value, domain);
        for part in &parts[1..] {
            let key_value: Vec<&str> = part.split('=').collect();
            let key = key_value[0].trim();
            if key == "Path" && key_value.len() > 1 {
                cookie.path = key_value[1].trim().to_string();
            } else if key == "Secure" {
                cookie.secure = true;
            } else if key == "HttpOnly" {
                cookie.http_only = true;
            } else if key == "SameSite" && key_value.len() > 1 {
                let val = key_value[1].trim();
                cookie.same_site = match val {
                    "Strict" => SameSite::Strict,
                    "Lax" => SameSite::Lax,
                    "None" => SameSite::None,
                    _ => SameSite::Default,
                };
            } else if key == "Max-Age" && key_value.len() > 1 {
                if let Ok(age) = key_value[1].trim().parse::<u64>() {
                    cookie.max_age = Some(age);
                }
            } else if key == "Expires" && key_value.len() > 1 {
                // Parse expires - simplified
            }
        }
        self.add_cookie(domain, cookie.clone());
        Some(cookie)
    }

    pub fn get_cookies(&self, domain: &str) -> Vec<&Cookie> {
        if let Some(cookies) = self.cookies.get(domain) {
            cookies.iter().collect()
        } else {
            Vec::new()
        }
    }

    pub fn get_cookies_for_request(&self, domain: &str, path: &str, secure: bool) -> Vec<&Cookie> {
        let mut result = Vec::new();
        if let Some(cookies) = self.cookies.get(domain) {
            for cookie in cookies {
                if cookie.path.starts_with(path) && (!cookie.secure || secure) {
                    result.push(cookie);
                }
            }
        }
        result
    }

    pub fn get_cookie_string(&self, domain: &str) -> String {
        let cookies = self.get_cookies(domain);
        cookies
            .iter()
            .map(|c| format!("{}={}", c.name, c.value))
            .collect::<Vec<String>>()
            .join("; ")
    }

    pub fn get_cookie_string_for_request(&self, domain: &str, path: &str, secure: bool) -> String {
        let cookies = self.get_cookies_for_request(domain, path, secure);
        cookies
            .iter()
            .map(|c| format!("{}={}", c.name, c.value))
            .collect::<Vec<String>>()
            .join("; ")
    }

    pub fn remove_cookie(&mut self, domain: &str, name: &str) {
        if let Some(cookies) = self.cookies.get_mut(domain) {
            cookies.retain(|c| c.name != name);
        }
    }

    pub fn clear_domain(&mut self, domain: &str) {
        self.cookies.remove(domain);
    }

    pub fn clear_all(&mut self) {
        self.cookies.clear();
    }

    pub fn contains_domain(&self, domain: &str) -> bool {
        self.cookies.contains_key(domain)
    }

    pub fn get_all_domains(&self) -> Vec<&String> {
        self.cookies.keys().collect()
    }
}

impl Default for CookieManager {
    fn default() -> Self {
        Self::new()
    }
}
