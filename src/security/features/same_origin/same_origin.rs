#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Origin {
    pub scheme: String,
    pub host: String,
    pub port: Option<u16>,
}

impl Origin {
    pub fn new(scheme: &str, host: &str, port: Option<u16>) -> Self {
        Origin {
            scheme: scheme.to_string(),
            host: host.to_string(),
            port,
        }
    }

    pub fn parse(url: &str) -> Option<Self> {
        let (scheme, rest) = url.split_once("://")?;
        let rest = rest.split(['/', '?', '#']).next()?;
        let (host, port) = match rest.split_once(':') {
            Some((h, p)) => {
                let port = p.parse::<u16>().ok()?;
                (h.to_string(), Some(port))
            }
            None => (rest.to_string(), default_port(scheme)),
        };
        Some(Origin::new(scheme, &host, port))
    }

    pub fn tuple(&self) -> (String, String, Option<u16>) {
        (self.scheme.clone(), self.host.clone(), self.port)
    }

    pub fn effective_port(&self) -> u16 {
        self.port
            .unwrap_or_else(|| default_port(&self.scheme).unwrap_or(80))
    }
}

fn default_port(scheme: &str) -> Option<u16> {
    match scheme {
        "http" => Some(80),
        "https" => Some(443),
        "ws" => Some(80),
        "wss" => Some(443),
        _ => None,
    }
}

pub struct SameOriginPolicy;

impl SameOriginPolicy {
    pub fn new() -> Self {
        SameOriginPolicy
    }

    pub fn is_same_origin(&self, a: &Origin, b: &Origin) -> bool {
        a.scheme == b.scheme && a.host == b.host && a.effective_port() == b.effective_port()
    }

    pub fn is_same_site(&self, a: &Origin, b: &Origin) -> bool {
        a.scheme == b.scheme
            && (a.host == b.host
                || a.host.ends_with(&format!(".{}", b.host))
                || b.host.ends_with(&format!(".{}", a.host)))
    }

    pub fn check_urls(&self, document_url: &str, resource_url: &str) -> bool {
        match (Origin::parse(document_url), Origin::parse(resource_url)) {
            (Some(a), Some(b)) => self.is_same_origin(&a, &b),
            _ => false,
        }
    }
}

impl Default for SameOriginPolicy {
    fn default() -> Self {
        SameOriginPolicy::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_origin_with_implicit_ports() {
        let origin = Origin::parse("https://example.com/path").unwrap();
        assert_eq!(origin.scheme, "https");
        assert_eq!(origin.host, "example.com");
        assert_eq!(origin.effective_port(), 443);
    }

    #[test]
    fn parses_explicit_port() {
        let origin = Origin::parse("http://example.com:8080/page").unwrap();
        assert_eq!(origin.port, Some(8080));
    }

    #[test]
    fn same_origin_matches() {
        let policy = SameOriginPolicy::new();
        let a = Origin::parse("https://example.com/a").unwrap();
        let b = Origin::parse("https://example.com/b").unwrap();
        assert!(policy.is_same_origin(&a, &b));
    }

    #[test]
    fn different_port_not_same_origin() {
        let policy = SameOriginPolicy::new();
        let a = Origin::parse("https://example.com/a").unwrap();
        let b = Origin::parse("https://example.com:8443/b").unwrap();
        assert!(!policy.is_same_origin(&a, &b));
    }

    #[test]
    fn different_scheme_not_same_origin() {
        let policy = SameOriginPolicy::new();
        let a = Origin::parse("https://example.com").unwrap();
        let b = Origin::parse("http://example.com").unwrap();
        assert!(!policy.is_same_origin(&a, &b));
    }

    #[test]
    fn same_site_allows_subdomains() {
        let policy = SameOriginPolicy::new();
        let a = Origin::parse("https://example.com").unwrap();
        let b = Origin::parse("https://api.example.com").unwrap();
        assert!(policy.is_same_site(&a, &b));
        assert!(!policy.is_same_origin(&a, &b));
    }

    #[test]
    fn check_urls_helper() {
        let policy = SameOriginPolicy::new();
        assert!(policy.check_urls("https://example.com/doc", "https://example.com/script.js"));
        assert!(!policy.check_urls("https://example.com/doc", "https://other.com/script.js"));
    }
}
