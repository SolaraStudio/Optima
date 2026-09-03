use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
pub struct FrameLimiter {
    pub target_fps: u32,
    pub frame_duration: Duration,
    pub last_frame_time: Option<Instant>,
    pub delta_time: Duration,
    pub total_frames: u64,
    pub missed_frames: u64,
    pub should_render: bool,
    pub vsync_enabled: bool,
    pub adaptive: bool,
    pub frame_budget_ns: u128,
    pub last_durations: Vec<Duration>,
    pub history_size: usize,
}

impl Default for FrameLimiter {
    fn default() -> Self {
        FrameLimiter {
            target_fps: 60,
            frame_duration: Duration::from_nanos(16_666_667),
            last_frame_time: None,
            delta_time: Duration::ZERO,
            total_frames: 0,
            missed_frames: 0,
            should_render: true,
            vsync_enabled: true,
            adaptive: false,
            frame_budget_ns: 16_666_667,
            last_durations: Vec::new(),
            history_size: 60,
        }
    }
}

impl FrameLimiter {
    pub fn new(target_fps: u32) -> Self {
        let fps = target_fps.max(1);
        FrameLimiter {
            target_fps: fps,
            frame_duration: Duration::from_nanos(1_000_000_000 / fps as u64),
            frame_budget_ns: 1_000_000_000 / fps as u128,
            ..Default::default()
        }
    }

    pub fn with_vsync(mut self, enabled: bool) -> Self {
        self.vsync_enabled = enabled;
        self
    }

    pub fn with_adaptive(mut self, enabled: bool) -> Self {
        self.adaptive = enabled;
        self
    }

    pub fn with_history_size(mut self, size: usize) -> Self {
        self.history_size = size.max(1);
        self
    }

    pub fn set_target_fps(&mut self, fps: u32) {
        let fps = fps.max(1);
        self.target_fps = fps;
        self.frame_duration = Duration::from_nanos(1_000_000_000 / fps as u64);
        self.frame_budget_ns = 1_000_000_000 / fps as u128;
    }

    pub fn begin_frame(&mut self) -> Instant {
        let now = Instant::now();
        if let Some(last) = self.last_frame_time {
            self.delta_time = now.duration_since(last);
            self.record_duration(self.delta_time);
        } else {
            self.delta_time = self.frame_duration;
        }
        self.last_frame_time = Some(now);
        self.total_frames += 1;
        self.should_render = true;
        now
    }

    pub fn end_frame(&mut self) {
        if let Some(start) = self.last_frame_time {
            let elapsed = start.elapsed();
            if elapsed < self.frame_duration {
                self.should_render = false;
            } else if elapsed > self.frame_duration * 2 {
                self.missed_frames += 1;
            }
        }
    }

    pub fn should_render_now(&self) -> bool {
        if !self.should_render {
            return false;
        }
        if let Some(last) = self.last_frame_time {
            let elapsed = last.elapsed();
            if self.vsync_enabled {
                elapsed >= self.frame_duration
            } else {
                elapsed >= self.frame_duration / 2
            }
        } else {
            true
        }
    }

    pub fn time_until_next_frame(&self) -> Duration {
        if let Some(last) = self.last_frame_time {
            let elapsed = last.elapsed();
            if elapsed >= self.frame_duration {
                Duration::ZERO
            } else {
                self.frame_duration - elapsed
            }
        } else {
            Duration::ZERO
        }
    }

    pub fn delta_time_secs(&self) -> f32 {
        self.delta_time.as_secs_f32()
    }

    pub fn delta_time_ms(&self) -> f32 {
        self.delta_time.as_secs_f32() * 1000.0
    }

    pub fn current_fps(&self) -> f32 {
        if self.delta_time.is_zero() {
            return 0.0;
        }
        1.0 / self.delta_time.as_secs_f32()
    }

    pub fn average_fps(&self) -> f32 {
        if self.last_durations.is_empty() {
            return 0.0;
        }
        let total: Duration = self.last_durations.iter().copied().sum();
        let avg = total / self.last_durations.len() as u32;
        if avg.is_zero() {
            0.0
        } else {
            1.0 / avg.as_secs_f32()
        }
    }

    fn record_duration(&mut self, dur: Duration) {
        self.last_durations.push(dur);
        if self.last_durations.len() > self.history_size {
            self.last_durations.remove(0);
        }
    }

    pub fn frame_budget_utilization(&self) -> f32 {
        if self.frame_budget_ns == 0 {
            return 0.0;
        }
        let avg_ns: u128 = if self.last_durations.is_empty() {
            self.delta_time.as_nanos()
        } else {
            let total: u128 = self.last_durations.iter().map(|d| d.as_nanos()).sum();
            total / self.last_durations.len() as u128
        };
        (avg_ns as f32 / self.frame_budget_ns as f32).clamp(0.0, 2.0)
    }

    pub fn missed_frame_percentage(&self) -> f32 {
        if self.total_frames == 0 {
            0.0
        } else {
            (self.missed_frames as f32 / self.total_frames as f32) * 100.0
        }
    }

    pub fn reset_stats(&mut self) {
        self.total_frames = 0;
        self.missed_frames = 0;
        self.last_durations.clear();
        self.last_frame_time = None;
    }

    pub fn adapt_to_performance(&mut self) {
        if !self.adaptive || self.last_durations.len() < 10 {
            return;
        }
        let util = self.frame_budget_utilization();
        if util > 0.95 && self.target_fps > 30 {
            self.set_target_fps(self.target_fps - 5);
        } else if util < 0.5 && self.target_fps < 120 {
            self.set_target_fps(self.target_fps + 5);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn test_frame_limiter_default() {
        let fl = FrameLimiter::default();
        assert_eq!(fl.target_fps, 60);
        assert_eq!(fl.total_frames, 0);
        assert_eq!(fl.missed_frames, 0);
        assert!(fl.vsync_enabled);
    }

    #[test]
    fn test_frame_limiter_new() {
        let fl = FrameLimiter::new(30);
        assert_eq!(fl.target_fps, 30);
        assert_eq!(fl.frame_budget_ns, 33_333_333);
        assert_eq!(fl.frame_duration, Duration::from_nanos(33_333_333));
    }

    #[test]
    fn test_frame_limiter_new_min_fps() {
        let fl = FrameLimiter::new(0);
        assert_eq!(fl.target_fps, 1);
    }

    #[test]
    fn test_set_target_fps() {
        let mut fl = FrameLimiter::new(60);
        fl.set_target_fps(120);
        assert_eq!(fl.target_fps, 120);
        assert_eq!(fl.frame_duration, Duration::from_nanos(8_333_333));
    }

    #[test]
    fn test_begin_end_frame() {
        let mut fl = FrameLimiter::new(60);
        let start = fl.begin_frame();
        assert_eq!(fl.total_frames, 1);
        assert!(fl.should_render);
        let _ = start;

        thread::sleep(Duration::from_millis(1));
        fl.end_frame();
    }

    #[test]
    fn test_delta_time() {
        let mut fl = FrameLimiter::new(60);
        fl.begin_frame();
        thread::sleep(Duration::from_millis(5));
        fl.begin_frame();
        assert!(fl.delta_time_ms() >= 4.0);
    }

    #[test]
    fn test_fps_calculation() {
        let mut fl = FrameLimiter::new(60);
        fl.begin_frame();
        thread::sleep(Duration::from_millis(16));
        fl.begin_frame();
        let fps = fl.current_fps();
        assert!(fps > 0.0 && fps < 200.0);
    }

    #[test]
    fn test_should_render_now() {
        let mut fl = FrameLimiter::new(60).with_vsync(false);
        fl.begin_frame();
        assert!(fl.should_render_now());
    }

    #[test]
    fn test_time_until_next_frame() {
        let mut fl = FrameLimiter::new(60);
        fl.begin_frame();
        let wait = fl.time_until_next_frame();
        assert!(wait <= Duration::from_millis(20));
    }

    #[test]
    fn test_frame_budget_utilization() {
        let mut fl = FrameLimiter::new(60);
        fl.begin_frame();
        fl.record_duration(Duration::from_nanos(10_000_000));
        let util = fl.frame_budget_utilization();
        assert!(util > 0.0 && util < 1.0);
    }

    #[test]
    fn test_stats_tracking() {
        let mut fl = FrameLimiter::new(60);
        for _ in 0..10 {
            fl.begin_frame();
        }
        assert_eq!(fl.total_frames, 10);

        fl.missed_frames = 3;
        assert!((fl.missed_frame_percentage() - 30.0).abs() < 0.01);

        fl.reset_stats();
        assert_eq!(fl.total_frames, 0);
        assert_eq!(fl.missed_frames, 0);
    }

    #[test]
    fn test_history_size() {
        let mut fl = FrameLimiter::new(60).with_history_size(5);
        for _ in 0..10 {
            fl.begin_frame();
        }
        assert!(fl.last_durations.len() <= 5);
    }

    #[test]
    fn test_builder_chain() {
        let fl = FrameLimiter::new(30)
            .with_vsync(false)
            .with_adaptive(true)
            .with_history_size(120);
        assert_eq!(fl.target_fps, 30);
        assert!(!fl.vsync_enabled);
        assert!(fl.adaptive);
        assert_eq!(fl.history_size, 120);
    }

    #[test]
    fn test_average_fps() {
        let mut fl = FrameLimiter::new(60);
        assert_eq!(fl.average_fps(), 0.0);

        fl.last_durations.push(Duration::from_millis(16));
        fl.last_durations.push(Duration::from_millis(17));
        let avg = fl.average_fps();
        assert!(avg > 55.0 && avg < 70.0);
    }

    #[test]
    fn test_adapt_to_performance() {
        let mut fl = FrameLimiter::new(60).with_adaptive(true);
        for _ in 0..20 {
            fl.last_durations.push(Duration::from_millis(20));
        }
        fl.adapt_to_performance();
        assert!(fl.target_fps < 60);

        let mut fl2 = FrameLimiter::new(30).with_adaptive(true);
        for _ in 0..20 {
            fl2.last_durations.push(Duration::from_millis(5));
        }
        fl2.adapt_to_performance();
        assert!(fl2.target_fps > 30);
    }
}
