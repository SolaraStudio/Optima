#[derive(Debug, Clone, PartialEq)]
pub enum SecurityPolicy {
    SameOrigin,
    NoCORS,
    CORS { origins: Vec<String> },
    Strict,
}

#[derive(Debug, Clone)]
pub struct SecurityManager {
    pub policy: SecurityPolicy,
    pub allowed_origins: Vec<String>,
    pub blocked_origins: Vec<String>,
    pub sandbox_enabled: bool,
}

impl Default for SecurityManager {
    fn default() -> Self {
        SecurityManager {
            policy: SecurityPolicy::SameOrigin,
            allowed_origins: Vec::new(),
            blocked_origins: Vec::new(),
            sandbox_enabled: true,
        }
    }
}

impl SecurityManager {
    pub fn new() -> Self { Self::default() }

    pub fn is_origin_allowed(&self, origin: &str) -> bool {
        if self.blocked_origins.iter().any(|b| origin.starts_with(b)) {
            return false;
        }
        if self.allowed_origins.is_empty() { return true; }
        self.allowed_origins.iter().any(|a| origin.starts_with(a))
    }

    pub fn allow_origin(&mut self, origin: &str) {
        if !self.allowed_origins.contains(&origin.to_string()) {
            self.allowed_origins.push(origin.to_string());
        }
    }

    pub fn block_origin(&mut self, origin: &str) {
        if !self.blocked_origins.contains(&origin.to_string()) {
            self.blocked_origins.push(origin.to_string());
        }
    }
}
