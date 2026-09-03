#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RedirectStatus {
    MovedPermanently,
    Found,
    TemporaryRedirect,
    PermanentRedirect,
}

impl RedirectStatus {
    pub fn from_code(code: u16) -> Option<Self> {
        match code {
            301 => Some(RedirectStatus::MovedPermanently),
            302 => Some(RedirectStatus::Found),
            307 => Some(RedirectStatus::TemporaryRedirect),
            308 => Some(RedirectStatus::PermanentRedirect),
            _ => None,
        }
    }

    pub fn code(&self) -> u16 {
        match self {
            RedirectStatus::MovedPermanently => 301,
            RedirectStatus::Found => 302,
            RedirectStatus::TemporaryRedirect => 307,
            RedirectStatus::PermanentRedirect => 308,
        }
    }

    pub fn is_permanent(&self) -> bool {
        matches!(
            self,
            RedirectStatus::MovedPermanently | RedirectStatus::PermanentRedirect
        )
    }

    pub fn preserves_method(&self) -> bool {
        matches!(
            self,
            RedirectStatus::TemporaryRedirect | RedirectStatus::PermanentRedirect
        )
    }
}

#[derive(Debug, Clone)]
pub struct RedirectHop {
    pub from: String,
    pub to: String,
    pub status: RedirectStatus,
    pub hop_number: usize,
}

pub struct RedirectTracker {
    pub max_hops: usize,
    pub history: Vec<RedirectHop>,
    pub visited: Vec<String>,
}

impl RedirectTracker {
    pub fn new(max_hops: usize) -> Self {
        RedirectTracker {
            max_hops,
            history: Vec::new(),
            visited: Vec::new(),
        }
    }

    pub fn record(&mut self, from: &str, to: &str, status: RedirectStatus) -> Result<(), String> {
        if self.history.len() >= self.max_hops {
            return Err(format!(
                "too many redirects: exceeded {} hops",
                self.max_hops
            ));
        }
        if self.visited.iter().any(|v| v == to) {
            return Err(format!("redirect loop detected at {}", to));
        }
        let hop_number = self.history.len() + 1;
        self.history.push(RedirectHop {
            from: from.to_string(),
            to: to.to_string(),
            status,
            hop_number,
        });
        self.visited.push(from.to_string());
        Ok(())
    }

    pub fn hops_used(&self) -> usize {
        self.history.len()
    }

    pub fn hops_remaining(&self) -> usize {
        self.max_hops.saturating_sub(self.history.len())
    }

    pub fn can_follow(&self) -> bool {
        self.history.len() < self.max_hops
    }

    pub fn current_url(&self) -> Option<&str> {
        self.history.last().map(|h| h.to.as_str())
    }

    pub fn has_redirected_off_origin(&self) -> bool {
        self.history.len() > 1
    }

    pub fn last_status(&self) -> Option<RedirectStatus> {
        self.history.last().map(|h| h.status)
    }
}

impl Default for RedirectTracker {
    fn default() -> Self {
        RedirectTracker::new(10)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn status_from_code() {
        assert_eq!(
            RedirectStatus::from_code(301),
            Some(RedirectStatus::MovedPermanently)
        );
        assert_eq!(
            RedirectStatus::from_code(308),
            Some(RedirectStatus::PermanentRedirect)
        );
        assert_eq!(RedirectStatus::from_code(404), None);
    }

    #[test]
    fn status_flags() {
        assert!(RedirectStatus::MovedPermanently.is_permanent());
        assert!(!RedirectStatus::Found.is_permanent());
        assert!(RedirectStatus::TemporaryRedirect.preserves_method());
        assert!(!RedirectStatus::Found.preserves_method());
    }

    #[test]
    fn records_hops() {
        let mut tracker = RedirectTracker::new(3);
        tracker
            .record("https://a.com", "https://b.com", RedirectStatus::Found)
            .unwrap();
        tracker
            .record(
                "https://b.com",
                "https://c.com",
                RedirectStatus::MovedPermanently,
            )
            .unwrap();
        assert_eq!(tracker.hops_used(), 2);
        assert_eq!(tracker.current_url(), Some("https://c.com"));
        assert_eq!(tracker.hops_remaining(), 1);
        assert!(tracker.can_follow());
    }

    #[test]
    fn rejects_loop() {
        let mut tracker = RedirectTracker::new(5);
        tracker
            .record("https://a.com", "https://b.com", RedirectStatus::Found)
            .unwrap();
        tracker
            .record("https://b.com", "https://a.com", RedirectStatus::Found)
            .unwrap();
        let result = tracker.record("https://a.com", "https://b.com", RedirectStatus::Found);
        assert!(result.is_err());
    }

    #[test]
    fn rejects_exceeding_max_hops() {
        let mut tracker = RedirectTracker::new(2);
        tracker
            .record("https://a.com", "https://b.com", RedirectStatus::Found)
            .unwrap();
        tracker
            .record("https://b.com", "https://c.com", RedirectStatus::Found)
            .unwrap();
        let result = tracker.record("https://c.com", "https://d.com", RedirectStatus::Found);
        assert!(result.is_err());
    }

    #[test]
    fn last_status_tracking() {
        let mut tracker = RedirectTracker::new(3);
        tracker
            .record(
                "https://a.com",
                "https://b.com",
                RedirectStatus::TemporaryRedirect,
            )
            .unwrap();
        assert_eq!(
            tracker.last_status(),
            Some(RedirectStatus::TemporaryRedirect)
        );
    }

    #[test]
    fn code_roundtrip() {
        assert_eq!(RedirectStatus::from_code(307).unwrap().code(), 307);
    }
}
