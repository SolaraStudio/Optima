use std::time::{Duration, Instant};

pub struct SyncController {
    start_time: Option<Instant>,
    position: Duration,
    duration: Duration,
    speed: f32,
    is_playing: bool,
    drift: f32,
}

impl SyncController {
    pub fn new() -> Self {
        Self {
            start_time: None,
            position: Duration::from_secs(0),
            duration: Duration::from_secs(0),
            speed: 1.0,
            is_playing: false,
            drift: 0.0,
        }
    }

    pub fn start(&mut self) {
        self.start_time = Some(Instant::now());
        self.is_playing = true;
    }

    pub fn pause(&mut self) {
        if let Some(start) = self.start_time {
            let elapsed = start.elapsed();
            self.position += elapsed.mul_f32(self.speed);
            self.start_time = None;
        }
        self.is_playing = false;
    }

    pub fn stop(&mut self) {
        self.start_time = None;
        self.position = Duration::from_secs(0);
        self.is_playing = false;
    }

    pub fn seek(&mut self, pos: Duration) {
        if self.is_playing {
            self.start_time = Some(Instant::now());
        }
        self.position = pos;
    }

    pub fn set_duration(&mut self, duration: Duration) {
        self.duration = duration;
    }

    pub fn set_speed(&mut self, speed: f32) {
        self.speed = speed.clamp(0.1, 4.0);
    }

    pub fn get_current_time(&self) -> Duration {
        if self.is_playing {
            if let Some(start) = self.start_time {
                let elapsed = start.elapsed().mul_f32(self.speed);
                let current = self.position + elapsed;
                if current > self.duration && self.duration > Duration::from_secs(0) {
                    return self.duration;
                }
                return current;
            }
        }
        self.position
    }

    pub fn get_progress(&self) -> f32 {
        if self.duration > Duration::from_secs(0) {
            self.get_current_time().as_secs_f32() / self.duration.as_secs_f32()
        } else {
            0.0
        }
    }

    pub fn get_speed(&self) -> f32 {
        self.speed
    }

    pub fn is_playing(&self) -> bool {
        self.is_playing
    }

    pub fn has_drift(&self) -> bool {
        self.drift.abs() > 0.01
    }

    pub fn reset_drift(&mut self) {
        self.drift = 0.0;
    }

    pub fn adjust_drift(&mut self, actual_position: Duration) {
        let expected = self.get_current_time();
        self.drift = actual_position.as_secs_f32() - expected.as_secs_f32();
    }
}

impl Default for SyncController {
    fn default() -> Self {
        Self::new()
    }
}
