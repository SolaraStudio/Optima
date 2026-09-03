#[derive(Debug, Clone)]
pub struct RetryPolicy {
    pub max_retries: u32,
    pub initial_delay: std::time::Duration,
    pub max_delay: std::time::Duration,
    pub backoff_factor: f32,
}

impl Default for RetryPolicy {
    fn default() -> Self {
        RetryPolicy {
            max_retries: 3,
            initial_delay: std::time::Duration::from_millis(100),
            max_delay: std::time::Duration::from_secs(30),
            backoff_factor: 2.0,
        }
    }
}

impl RetryPolicy {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn delay_for_attempt(&self, attempt: u32) -> std::time::Duration {
        let factor = self.backoff_factor.powi(attempt as i32);
        let delay_ms = (self.initial_delay.as_millis() as f32 * factor) as u64;
        let max_ms = self.max_delay.as_millis() as u64;
        std::time::Duration::from_millis(delay_ms.min(max_ms))
    }

    pub fn should_retry(&self, attempt: u32, status: u16) -> bool {
        attempt < self.max_retries && (status >= 500 || status == 429 || status == 0)
    }
}
