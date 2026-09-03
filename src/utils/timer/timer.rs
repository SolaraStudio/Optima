use std::time::{Instant, Duration};

pub struct Timer {
    start: Instant,
    lap_start: Instant,
}

impl Timer {
    pub fn start() -> Self {
        let now = Instant::now();
        Timer {
            start: now,
            lap_start: now,
        }
    }

    pub fn elapsed_ms(&self) -> u64 {
        self.start.elapsed().as_millis() as u64
    }

    pub fn elapsed_secs(&self) -> f64 {
        self.start.elapsed().as_secs_f64()
    }

    pub fn elapsed_nanos(&self) -> u64 {
        self.start.elapsed().as_nanos() as u64
    }

    pub fn lap(&mut self) -> u64 {
        let now = Instant::now();
        let elapsed = now.duration_since(self.lap_start).as_millis() as u64;
        self.lap_start = now;
        elapsed
    }

    pub fn reset(&mut self) {
        self.start = Instant::now();
        self.lap_start = self.start;
    }
}

pub struct Stopwatch {
    start: Option<Instant>,
    accumulated: Duration,
}

impl Stopwatch {
    pub fn new() -> Self {
        Stopwatch {
            start: None,
            accumulated: Duration::from_secs(0),
        }
    }

    pub fn start(&mut self) {
        if self.start.is_none() {
            self.start = Some(Instant::now());
        }
    }

    pub fn stop(&mut self) {
        if let Some(start) = self.start.take() {
            self.accumulated += start.elapsed();
        }
    }

    pub fn reset(&mut self) {
        self.start = None;
        self.accumulated = Duration::from_secs(0);
    }

    pub fn elapsed_ms(&self) -> u64 {
        let mut total = self.accumulated;
        if let Some(start) = self.start {
            total += start.elapsed();
        }
        total.as_millis() as u64
    }

    pub fn elapsed_secs(&self) -> f64 {
        let mut total = self.accumulated;
        if let Some(start) = self.start {
            total += start.elapsed();
        }
        total.as_secs_f64()
    }

    pub fn is_running(&self) -> bool {
        self.start.is_some()
    }
}
