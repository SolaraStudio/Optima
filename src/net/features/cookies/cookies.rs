#[derive(Debug, Clone, PartialEq)]
pub struct CookieSpec {
    pub name: String,
    pub value: String,
    pub domain: String,
    pub path: String,
    pub secure: bool,
    pub http_only: bool,
    pub same_site: Option<String>,
    pub max_age: Option<u64>,
}

impl CookieSpec {
    pub fn new(name: &str, value: &str, domain: &str) -> Self {
        CookieSpec {
            name: name.to_string(),
            value: value.to_string(),
            domain: domain.to_string(),
            path: "/".to_string(),
            secure: false,
            http_only: false,
            same_site: None,
            max_age: None,
        }
    }
}

#[derive(Default)]
pub struct CookieJar {
    cookies: Vec<CookieSpec>,
}

impl CookieJar {
    pub fn new() -> Self {
        CookieJar { cookies: Vec::new() }
    }

    pub fn set(&mut self, cookie: CookieSpec) {
        if let Some(existing) = self.cookies.iter_mut()
            .find(|c| c.name == cookie.name && c.domain == cookie.domain && c.path == cookie.path)
        {
            *existing = cookie;
        } else {
            self.cookies.push(cookie);
        }
    }

    pub fn get(&self, name: &str, domain: &str, path: &str) -> Option<&CookieSpec> {
        self.cookies.iter().find(|c| {
            c.name == name
                && self.domain_matches(domain, &c.domain)
                && path.starts_with(&c.path)
        })
    }

    pub fn get_all(&self, domain: &str, _path: &str, secure: bool) -> Vec<&CookieSpec> {
        self.cookies.iter()
            .filter(|c| {
                self.domain_matches(domain, &c.domain)
                    && (!c.secure || secure)
            })
            .collect()
    }

    pub fn remove(&mut self, name: &str, domain: &str, path: &str) -> bool {
        let before = self.cookies.len();
        self.cookies.retain(|c| {
            !(c.name == name && c.domain == domain && c.path == path)
        });
        self.cookies.len() != before
    }

    pub fn has(&self, name: &str, domain: &str, path: &str) -> bool {
        self.get(name, domain, path).is_some()
    }

    pub fn clear(&mut self, domain: &str) {
        self.cookies.retain(|c| c.domain != domain);
    }

    pub fn clear_all(&mut self) {
        self.cookies.clear();
    }

    pub fn len(&self) -> usize {
        self.cookies.len()
    }

    pub fn is_empty(&self) -> bool {
        self.cookies.is_empty()
    }

    fn domain_matches(&self, request_domain: &str, cookie_domain: &str) -> bool {
        request_domain == cookie_domain
            || request_domain.ends_with(&format!(".{}", cookie_domain))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_and_get_cookie() {
        let mut jar = CookieJar::new();
        jar.set(CookieSpec::new("session", "abc", "example.com"));
        assert!(jar.has("session", "example.com", "/"));
        let cookie = jar.get("session", "example.com", "/").unwrap();
        assert_eq!(cookie.value, "abc");
    }

    #[test]
    fn domain_matches_subdomain() {
        let mut jar = CookieJar::new();
        jar.set(CookieSpec::new("sid", "123", "example.com"));
        assert!(jar.has("sid", "sub.example.com", "/"));
        assert!(!jar.has("sid", "notexample.com", "/"));
    }

    #[test]
    fn path_must_prefix_match() {
        let mut jar = CookieJar::new();
        let mut cookie = CookieSpec::new("auth", "yes", "example.com");
        cookie.path = "/account".to_string();
        jar.set(cookie);
        assert!(jar.has("auth", "example.com", "/account/profile"));
        assert!(!jar.has("auth", "example.com", "/"));
    }

    #[test]
    fn set_overwrites_same_identity() {
        let mut jar = CookieJar::new();
        jar.set(CookieSpec::new("k", "v1", "example.com"));
        jar.set(CookieSpec::new("k", "v2", "example.com"));
        assert_eq!(jar.len(), 1);
        assert_eq!(jar.get("k", "example.com", "/").unwrap().value, "v2");
    }

    #[test]
    fn secure_cookie_only_sent_over_secure() {
        let mut jar = CookieJar::new();
        let mut c = CookieSpec::new("ssid", "x", "example.com");
        c.secure = true;
        jar.set(c);
        jar.set(CookieSpec::new("plain", "y", "example.com"));
        let over_https = jar.get_all("example.com", "/", true);
        let over_http = jar.get_all("example.com", "/", false);
        assert_eq!(over_https.len(), 2);
        assert_eq!(over_http.len(), 1);
    }

    #[test]
    fn remove_deletes_cookie() {
        let mut jar = CookieJar::new();
        jar.set(CookieSpec::new("k", "v", "example.com"));
        assert!(jar.remove("k", "example.com", "/"));
        assert!(!jar.has("k", "example.com", "/"));
        assert!(jar.is_empty());
    }

    #[test]
    fn clear_domain_cookies() {
        let mut jar = CookieJar::new();
        jar.set(CookieSpec::new("a", "1", "example.com"));
        jar.set(CookieSpec::new("b", "2", "other.org"));
        jar.clear("example.com");
        assert_eq!(jar.len(), 1);
        assert!(jar.has("b", "other.org", "/"));
    }
}
