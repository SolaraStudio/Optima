use std::time::Duration;

#[derive(Debug, Clone)]
pub struct TimeoutConfig {
    pub connect: Duration,
    pub read: Duration,
    pub write: Duration,
    pub total: Duration,
}

impl Default for TimeoutConfig {
    fn default() -> Self {
        TimeoutConfig {
            connect: Duration::from_secs(10),
            read: Duration::from_secs(30),
            write: Duration::from_secs(30),
            total: Duration::from_secs(60),
        }
    }
}

impl TimeoutConfig {
    pub fn new() -> Self { Self::default() }
    pub fn with_connect(mut self, d: Duration) -> Self { self.connect = d; self }
    pub fn with_read(mut self, d: Duration) -> Self { self.read = d; self }
    pub fn with_total(mut self, d: Duration) -> Self { self.total = d; self }
}
