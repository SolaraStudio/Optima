#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EasingFunction {
    Linear,
    EaseIn,
    EaseOut,
    EaseInOut,
    CubicIn,
    CubicOut,
    CubicInOut,
    BounceOut,
    ElasticOut,
}

impl Default for EasingFunction {
    fn default() -> Self {
        EasingFunction::EaseInOut
    }
}

impl EasingFunction {
    pub fn evaluate(&self, t: f32) -> f32 {
        let t = t.clamp(0.0, 1.0);
        match self {
            EasingFunction::Linear => t,
            EasingFunction::EaseIn => t * t,
            EasingFunction::EaseOut => t * (2.0 - t),
            EasingFunction::EaseInOut => {
                if t < 0.5 {
                    2.0 * t * t
                } else {
                    -1.0 + (4.0 - 2.0 * t) * t
                }
            }
            EasingFunction::CubicIn => t * t * t,
            EasingFunction::CubicOut => {
                let t1 = t - 1.0;
                t1 * t1 * t1 + 1.0
            }
            EasingFunction::CubicInOut => {
                if t < 0.5 {
                    4.0 * t * t * t
                } else {
                    let t1 = 2.0 * t - 2.0;
                    0.5 * t1 * t1 * t1 + 1.0
                }
            }
            EasingFunction::BounceOut => {
                if t < 1.0 / 2.75 {
                    7.5625 * t * t
                } else if t < 2.0 / 2.75 {
                    let t = t - 1.5 / 2.75;
                    7.5625 * t * t + 0.75
                } else if t < 2.5 / 2.75 {
                    let t = t - 2.25 / 2.75;
                    7.5625 * t * t + 0.9375
                } else {
                    let t = t - 2.625 / 2.75;
                    7.5625 * t * t + 0.984375
                }
            }
            EasingFunction::ElasticOut => {
                if t == 0.0 || t == 1.0 {
                    t
                } else {
                    let p = 0.3;
                    let s = p / 4.0;
                    let t = t - 1.0;
                    -(pow2(10.0 * t) * sin_approx((t - s) * std::f32::consts::TAU / p))
                }
            }
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            EasingFunction::Linear => "linear",
            EasingFunction::EaseIn => "ease-in",
            EasingFunction::EaseOut => "ease-out",
            EasingFunction::EaseInOut => "ease-in-out",
            EasingFunction::CubicIn => "cubic-in",
            EasingFunction::CubicOut => "cubic-out",
            EasingFunction::CubicInOut => "cubic-in-out",
            EasingFunction::BounceOut => "bounce-out",
            EasingFunction::ElasticOut => "elastic-out",
        }
    }
}

fn pow2(x: f32) -> f32 {
    (x * std::f32::consts::LN_2).exp()
}

fn sin_approx(x: f32) -> f32 {
    let x = x % (2.0 * std::f32::consts::PI);
    if x < 0.0 {
        return -sin_approx(-x);
    }
    if x > std::f32::consts::PI {
        return -sin_approx(x - std::f32::consts::PI);
    }
    let x2 = x * x;
    let x4 = x2 * x2;
    let x6 = x4 * x2;
    x - x * x2 / 6.0 + x * x4 / 120.0 - x * x6 / 5040.0
}

#[derive(Debug, Clone)]
pub struct SmoothScrollState {
    pub current_position: f32,
    pub target_position: f32,
    pub velocity: f32,
    pub max_velocity: f32,
    pub friction: f32,
    pub easing: EasingFunction,
    pub duration_ms: f32,
    pub elapsed_ms: f32,
    pub is_animating: bool,
    pub start_position: f32,
    pub bounce_back: bool,
    pub overshoot_limit: f32,
}

impl Default for SmoothScrollState {
    fn default() -> Self {
        SmoothScrollState {
            current_position: 0.0,
            target_position: 0.0,
            velocity: 0.0,
            max_velocity: 5000.0,
            friction: 0.95,
            easing: EasingFunction::EaseInOut,
            duration_ms: 300.0,
            elapsed_ms: 0.0,
            is_animating: false,
            start_position: 0.0,
            bounce_back: false,
            overshoot_limit: 50.0,
        }
    }
}

impl SmoothScrollState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_easing(mut self, easing: EasingFunction) -> Self {
        self.easing = easing;
        self
    }

    pub fn with_duration(mut self, ms: f32) -> Self {
        self.duration_ms = ms.max(1.0);
        self
    }

    pub fn with_max_velocity(mut self, vel: f32) -> Self {
        self.max_velocity = vel;
        self
    }

    pub fn with_friction(mut self, friction: f32) -> Self {
        self.friction = friction.clamp(0.8, 1.0);
        self
    }

    pub fn with_bounce_back(mut self, enabled: bool, limit: f32) -> Self {
        self.bounce_back = enabled;
        self.overshoot_limit = limit;
        self
    }

    pub fn scroll_to(&mut self, target: f32) {
        self.target_position = target;
        self.start_position = self.current_position;
        self.elapsed_ms = 0.0;
        self.is_animating = true;
    }

    pub fn scroll_by(&mut self, delta: f32) {
        self.scroll_to(self.target_position + delta);
    }

    pub fn scroll_with_velocity(&mut self, velocity: f32) {
        self.velocity = velocity.clamp(-self.max_velocity, self.max_velocity);
        self.target_position = self.current_position + velocity * 0.3;
        self.start_position = self.current_position;
        self.elapsed_ms = 0.0;
        self.is_animating = true;
    }

    pub fn update(&mut self, dt_ms: f32) {
        if !self.is_animating {
            return;
        }

        self.elapsed_ms += dt_ms;

        if self.duration_ms > 0.0 && self.elapsed_ms >= self.duration_ms {
            self.current_position = self.target_position;
            self.velocity = 0.0;
            self.is_animating = false;
            return;
        }

        if self.duration_ms > 0.0 {
            let t = self.elapsed_ms / self.duration_ms;
            let eased_t = self.easing.evaluate(t);
            let prev = self.current_position;
            self.current_position =
                self.start_position + (self.target_position - self.start_position) * eased_t;
            self.velocity = if dt_ms > 0.0 {
                (self.current_position - prev) * (1000.0 / dt_ms)
            } else {
                0.0
            };
        } else {
            let diff = self.target_position - self.current_position;
            self.current_position += diff.min(self.max_velocity * dt_ms / 1000.0);
            self.velocity = if dt_ms > 0.0 {
                diff * (1000.0 / dt_ms)
            } else {
                0.0
            };
            if diff.abs() < 0.5 {
                self.current_position = self.target_position;
                self.velocity = 0.0;
                self.is_animating = false;
            }
        }
    }

    pub fn snap_to(&mut self, position: f32) {
        self.current_position = position;
        self.target_position = position;
        self.velocity = 0.0;
        self.is_animating = false;
    }

    pub fn stop(&mut self) {
        self.target_position = self.current_position;
        self.velocity = 0.0;
        self.is_animating = false;
    }

    pub fn progress(&self) -> f32 {
        if self.duration_ms > 0.0 {
            (self.elapsed_ms / self.duration_ms).clamp(0.0, 1.0)
        } else {
            1.0
        }
    }

    pub fn remaining_distance(&self) -> f32 {
        (self.target_position - self.current_position).abs()
    }

    pub fn is_at_target(&self) -> bool {
        self.remaining_distance() < 0.5 && !self.is_animating
    }

    pub fn clamp_position(&mut self, min: f32, max: f32) {
        self.current_position = self.current_position.clamp(min, max);
        self.target_position = self.target_position.clamp(min, max);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_easing_linear() {
        let e = EasingFunction::Linear;
        assert!((e.evaluate(0.0) - 0.0).abs() < 0.001);
        assert!((e.evaluate(0.5) - 0.5).abs() < 0.001);
        assert!((e.evaluate(1.0) - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_easing_ease_in() {
        let e = EasingFunction::EaseIn;
        assert!((e.evaluate(0.0)).abs() < 0.001);
        assert!((e.evaluate(0.5) - 0.25).abs() < 0.001);
        assert!((e.evaluate(1.0) - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_easing_ease_out() {
        let e = EasingFunction::EaseOut;
        assert!((e.evaluate(0.0)).abs() < 0.001);
        assert!((e.evaluate(0.5) - 0.75).abs() < 0.001);
        assert!((e.evaluate(1.0) - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_easing_clamp() {
        let e = EasingFunction::EaseInOut;
        assert!((e.evaluate(-1.0)).abs() < 0.001);
        assert!((e.evaluate(2.0) - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_easing_names() {
        assert_eq!(EasingFunction::Linear.name(), "linear");
        assert_eq!(EasingFunction::BounceOut.name(), "bounce-out");
        assert_eq!(EasingFunction::ElasticOut.name(), "elastic-out");
    }

    #[test]
    fn test_smooth_scroll_default() {
        let s = SmoothScrollState::default();
        assert_eq!(s.current_position, 0.0);
        assert_eq!(s.target_position, 0.0);
        assert!(!s.is_animating);
        assert_eq!(s.max_velocity, 5000.0);
    }

    #[test]
    fn test_smooth_scroll_to() {
        let mut s = SmoothScrollState::new();
        s.scroll_to(100.0);
        assert!(s.is_animating);
        assert_eq!(s.target_position, 100.0);
        assert_eq!(s.start_position, 0.0);
    }

    #[test]
    fn test_smooth_scroll_by() {
        let mut s = SmoothScrollState::new();
        s.current_position = 50.0;
        s.scroll_by(25.0);
        assert_eq!(s.target_position, 75.0);
    }

    #[test]
    fn test_smooth_scroll_update() {
        let mut s = SmoothScrollState::with_duration(100.0);
        s.scroll_to(100.0);

        for _ in 0..100 {
            s.update(1.0);
        }

        assert!((s.current_position - 100.0).abs() < 1.0);
        assert!(!s.is_animating);
    }

    #[test]
    fn test_smooth_scroll_snap() {
        let mut s = SmoothScrollState::new();
        s.scroll_to(500.0);
        assert!(s.is_animating);

        s.snap_to(250.0);
        assert!(!s.is_animating);
        assert_eq!(s.current_position, 250.0);
        assert_eq!(s.target_position, 250.0);
    }

    #[test]
    fn test_smooth_scroll_stop() {
        let mut s = SmoothScrollState::new();
        s.scroll_to(1000.0);
        s.update(16.0);
        assert!(s.is_animating);

        s.stop();
        assert!(!s.is_animating);
        assert_eq!(s.target_position, s.current_position);
    }

    #[test]
    fn test_smooth_scroll_progress() {
        let mut s = SmoothScrollState::with_duration(200.0);
        s.scroll_to(100.0);

        assert_eq!(s.progress(), 0.0);
        s.update(100.0);
        assert!((s.progress() - 0.5).abs() < 0.01);
        s.update(100.0);
        assert!((s.progress() - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_smooth_scroll_velocity() {
        let mut s = SmoothScrollState::new();
        s.scroll_with_velocity(1000.0);
        assert!(s.is_animating);
        assert_eq!(s.velocity, 1000.0);
    }

    #[test]
    fn test_smooth_scroll_clamp() {
        let mut s = SmoothScrollState::new();
        s.scroll_to(500.0);
        s.clamp_position(0.0, 200.0);
        assert_eq!(s.target_position, 200.0);
    }

    #[test]
    fn test_smooth_scroll_bounce_back() {
        let mut s = SmoothScrollState::new().with_bounce_back(true, 100.0);
        assert!(s.bounce_back);
        assert_eq!(s.overshoot_limit, 100.0);
    }
}
