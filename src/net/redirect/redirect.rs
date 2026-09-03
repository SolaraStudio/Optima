#[derive(Debug, Clone)]
pub struct RedirectPolicy {
    pub max_redirects: u32,
    pub follow_redirects: bool,
    pub trusted_domains: Vec<String>,
}

impl Default for RedirectPolicy {
    fn default() -> Self {
        RedirectPolicy {
            max_redirects: 10,
            follow_redirects: true,
            trusted_domains: Vec::new(),
        }
    }
}

impl RedirectPolicy {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn should_follow(&self, _url: &str, redirect_count: u32) -> bool {
        self.follow_redirects && redirect_count < self.max_redirects
    }

    pub fn is_trusted(&self, domain: &str) -> bool {
        self.trusted_domains.is_empty() || self.trusted_domains.iter().any(|d| domain.ends_with(d))
    }
}
