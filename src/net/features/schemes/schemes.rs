use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UrlScheme {
    Optimus,
    File,
    Data,
    Http,
    Https,
    Unknown,
}

impl UrlScheme {
    pub fn from_str(s: &str) -> Self {
        match s {
            "optimus" => UrlScheme::Optimus,
            "file" => UrlScheme::File,
            "data" => UrlScheme::Data,
            "http" => UrlScheme::Http,
            "https" => UrlScheme::Https,
            _ => UrlScheme::Unknown,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            UrlScheme::Optimus => "optimus",
            UrlScheme::File => "file",
            UrlScheme::Data => "data",
            UrlScheme::Http => "http",
            UrlScheme::Https => "https",
            UrlScheme::Unknown => "unknown",
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ParsedUrl {
    pub scheme: UrlScheme,
    pub host: Option<String>,
    pub path: String,
    pub query: Option<String>,
    pub raw: String,
}

impl ParsedUrl {
    pub fn new(raw: &str) -> Option<Self> {
        let (scheme_part, rest) = raw.split_once("://")?;
        if scheme_part.contains(':') {
            let (data_scheme, data_rest) = raw.split_once(':')?;
            if data_scheme == "data" {
                return Some(ParsedUrl {
                    scheme: UrlScheme::Data,
                    host: None,
                    path: data_rest.to_string(),
                    query: None,
                    raw: raw.to_string(),
                });
            }
        }
        let scheme = UrlScheme::from_str(scheme_part);
        let (authority, path) = match rest.split_once('/') {
            Some((a, p)) => (a, format!("/{}", p)),
            None => (rest, "/".to_string()),
        };
        let (host, query) = match authority.split_once('?') {
            Some((h, q)) => (Some(h.to_string()), Some(q.to_string())),
            None => (Some(authority.to_string()), None),
        };
        Some(ParsedUrl { scheme, host, path, query, raw: raw.to_string() })
    }
}

#[derive(Debug, Clone)]
pub struct SchemeHandler {
    pub name: UrlScheme,
    pub handler: fn(&ParsedUrl) -> Result<Vec<u8>, String>,
}

pub struct SchemeRegistry {
    handlers: HashMap<String, SchemeHandler>,
}

impl SchemeRegistry {
    pub fn new() -> Self {
        let mut registry = SchemeRegistry { handlers: HashMap::new() };
        registry.register(UrlScheme::Optimus, |url| {
            Ok(format!("optimus:{}", url.path).into_bytes())
        });
        registry.register(UrlScheme::File, |_url| {
            Err("file scheme requires filesystem access".to_string())
        });
        registry.register(UrlScheme::Data, |url| {
            if let Some((_, payload)) = url.path.split_once(',') {
                Ok(payload.as_bytes().to_vec())
            } else {
                Err("malformed data url".to_string())
            }
        });
        registry
    }

    pub fn register(&mut self, scheme: UrlScheme, handler: fn(&ParsedUrl) -> Result<Vec<u8>, String>) {
        self.handlers.insert(scheme.as_str().to_string(), SchemeHandler { name: scheme, handler });
    }

    pub fn unregister(&mut self, scheme: UrlScheme) {
        self.handlers.remove(scheme.as_str());
    }

    pub fn has(&self, scheme: UrlScheme) -> bool {
        self.handlers.contains_key(scheme.as_str())
    }

    pub fn resolve(&self, url: &ParsedUrl) -> Result<Vec<u8>, String> {
        match self.handlers.get(url.scheme.as_str()) {
            Some(entry) => (entry.handler)(url),
            None => Err(format!("no handler for scheme {}", url.scheme.as_str())),
        }
    }

    pub fn parse(&self, raw: &str) -> Option<ParsedUrl> {
        ParsedUrl::new(raw)
    }
}

impl Default for SchemeRegistry {
    fn default() -> Self {
        SchemeRegistry::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_optimus_url() {
        let parsed = ParsedUrl::new("optimus://home/index.html").unwrap();
        assert_eq!(parsed.scheme, UrlScheme::Optimus);
        assert_eq!(parsed.host.as_deref(), Some("home"));
        assert_eq!(parsed.path, "/index.html");
    }

    #[test]
    fn parses_data_url() {
        let parsed = ParsedUrl::new("data:text/html,hello").unwrap();
        assert_eq!(parsed.scheme, UrlScheme::Data);
    }

    #[test]
    fn registry_has_builtin_schemes() {
        let registry = SchemeRegistry::new();
        assert!(registry.has(UrlScheme::Optimus));
        assert!(registry.has(UrlScheme::File));
        assert!(registry.has(UrlScheme::Data));
    }

    #[test]
    fn resolve_data_url() {
        let registry = SchemeRegistry::new();
        let parsed = ParsedUrl::new("data:text/plain,optima").unwrap();
        let result = registry.resolve(&parsed).unwrap();
        assert_eq!(result, b"optima");
    }

    #[test]
    fn unregister_scheme() {
        let mut registry = SchemeRegistry::new();
        registry.unregister(UrlScheme::Data);
        assert!(!registry.has(UrlScheme::Data));
    }

    #[test]
    fn custom_handler_registration() {
        let mut registry = SchemeRegistry::new();
        registry.register(UrlScheme::Unknown, |url| {
            Ok(format!("custom:{}", url.path).into_bytes())
        });
        assert!(registry.has(UrlScheme::Unknown));
    }
}
