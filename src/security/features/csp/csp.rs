use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum SourceType {
    Host(String),
    Scheme(String),
    Keyword,
    None,
    UnsafeInline,
    UnsafeEval,
    SelfOrigin,
    Wildcard,
}

#[derive(Debug, Clone)]
pub struct Directive {
    pub name: String,
    pub sources: Vec<String>,
}

impl Directive {
    pub fn new(name: &str, sources: Vec<String>) -> Self {
        Directive {
            name: name.to_string(),
            sources,
        }
    }

    pub fn allows(&self, source: &str) -> bool {
        for s in &self.sources {
            if s == "*" {
                return true;
            }
            if s == "'self'" && source.starts_with("//") {
                return true;
            }
            if s == source || source.starts_with(s) {
                return true;
            }
        }
        false
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Policy {
    pub directives: HashMap<String, Vec<String>>,
}

pub struct CspParser {
    pub policy: Policy,
    pub report_uri: Option<String>,
}

impl CspParser {
    pub fn new() -> Self {
        CspParser {
            policy: Policy {
                directives: HashMap::new(),
            },
            report_uri: None,
        }
    }

    pub fn parse(&mut self, header: &str) -> Result<(), String> {
        self.policy.directives.clear();
        for part in header.split(';') {
            let trimmed = part.trim();
            if trimmed.is_empty() {
                continue;
            }
            let mut tokens = trimmed.split_whitespace();
            let directive_name = tokens
                .next()
                .ok_or("missing directive name")?
                .to_lowercase();
            let sources: Vec<String> = tokens.map(|s| s.to_string()).collect();
            if directive_name == "report-uri" {
                self.report_uri = sources.first().cloned();
            } else {
                self.policy
                    .directives
                    .insert(directive_name.clone(), sources.clone());
            }
        }
        Ok(())
    }

    pub fn get_directive(&self, name: &str) -> Option<&Vec<String>> {
        self.policy.directives.get(name)
    }

    pub fn has_directive(&self, name: &str) -> bool {
        self.policy.directives.contains_key(name)
    }

    pub fn allows(&self, directive: &str, source: &str) -> bool {
        match self.policy.directives.get(directive) {
            Some(sources) => Directive::new(directive, sources.clone()).allows(source),
            None => true,
        }
    }

    pub fn allows_script(&self, source: &str) -> bool {
        if self.has_directive("script-src") {
            self.allows("script-src", source)
        } else {
            self.allows("default-src", source)
        }
    }

    pub fn allows_connect(&self, source: &str) -> bool {
        if self.has_directive("connect-src") {
            self.allows("connect-src", source)
        } else {
            self.allows("default-src", source)
        }
    }

    pub fn allows_style(&self, source: &str) -> bool {
        if self.has_directive("style-src") {
            self.allows("style-src", source)
        } else {
            self.allows("default-src", source)
        }
    }

    pub fn disallows(&self, _source: &str) -> bool {
        self.get_directive("default-src")
            .is_some_and(|s| s.contains(&"'none'".to_string()))
    }

    pub fn allows_inline_script(&self) -> bool {
        self.get_directive("script-src").is_some_and(|s| {
            s.iter()
                .any(|v| v == "'unsafe-inline'" || v == "*" || v == "'self'")
        })
    }

    pub fn allows_inline_style(&self) -> bool {
        self.get_directive("style-src").is_some_and(|s| {
            s.iter()
                .any(|v| v == "'unsafe-inline'" || v == "*" || v == "'self'")
        })
    }
}

impl Default for CspParser {
    fn default() -> Self {
        CspParser::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_directives() {
        let mut parser = CspParser::new();
        parser.parse("default-src 'self'; script-src 'self' https://cdn.example.com; style-src 'unsafe-inline'").unwrap();
        assert!(parser.has_directive("default-src"));
        assert!(parser.has_directive("script-src"));
        assert!(
            parser
                .get_directive("script-src")
                .unwrap()
                .contains(&"https://cdn.example.com".to_string())
        );
    }

    #[test]
    fn parse_report_uri() {
        let mut parser = CspParser::new();
        parser
            .parse("default-src 'self'; report-uri /csp-report")
            .unwrap();
        assert_eq!(parser.report_uri.as_deref(), Some("/csp-report"));
    }

    #[test]
    fn allows_host_source() {
        let mut parser = CspParser::new();
        parser
            .parse("default-src 'self'; script-src https://cdn.example.com")
            .unwrap();
        assert!(parser.allows_script("https://cdn.example.com/lib.js"));
        assert!(!parser.allows_script("https://evil.com/x.js"));
    }

    #[test]
    fn allows_wildcard() {
        let mut parser = CspParser::new();
        parser.parse("connect-src *").unwrap();
        assert!(parser.allows_connect("https://anything.com"));
    }

    #[test]
    fn no_policy_allows_everything() {
        let parser = CspParser::new();
        assert!(parser.allows_script("https://anywhere.com"));
    }

    #[test]
    fn inline_script_check() {
        let mut parser = CspParser::new();
        parser.parse("script-src 'unsafe-inline'").unwrap();
        assert!(parser.allows_inline_script());
    }

    #[test]
    fn none_disallows() {
        let mut parser = CspParser::new();
        parser.parse("default-src 'none'").unwrap();
        assert!(parser.disallows("https://x.com"));
    }
}
