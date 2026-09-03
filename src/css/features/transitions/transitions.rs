#[derive(Debug, Clone, PartialEq)]
pub enum EasingFunction {
    Linear,
    Ease,
    EaseIn,
    EaseOut,
    EaseInOut,
    CubicBezier(f32, f32, f32, f32),
    Steps(u32, StepTiming),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum StepTiming {
    Start,
    End,
}

#[derive(Debug, Clone)]
pub struct TransitionProperty {
    pub name: String,
    pub duration_ms: f32,
    pub delay_ms: f32,
    pub easing: EasingFunction,
}

impl TransitionProperty {
    pub fn new(name: &str, duration_ms: f32) -> Self {
        TransitionProperty {
            name: name.to_string(),
            duration_ms,
            delay_ms: 0.0,
            easing: EasingFunction::Ease,
        }
    }

    pub fn with_delay(mut self, delay_ms: f32) -> Self {
        self.delay_ms = delay_ms;
        self
    }

    pub fn with_easing(mut self, easing: EasingFunction) -> Self {
        self.easing = easing;
        self
    }

    pub fn total_duration_ms(&self) -> f32 {
        self.duration_ms + self.delay_ms
    }

    pub fn progress_at(&self, elapsed_ms: f32) -> f32 {
        if elapsed_ms < self.delay_ms {
            return 0.0;
        }
        let active_time = elapsed_ms - self.delay_ms;
        let raw = if self.duration_ms > 0.0 {
            (active_time / self.duration_ms).min(1.0)
        } else {
            1.0
        };
        self.easing.apply(raw)
    }
}

impl EasingFunction {
    pub fn apply(&self, t: f32) -> f32 {
        let t = t.clamp(0.0, 1.0);
        match self {
            EasingFunction::Linear => t,
            EasingFunction::Ease => cubic_bezier(0.25, 0.1, 0.25, 1.0, t),
            EasingFunction::EaseIn => cubic_bezier(0.42, 0.0, 1.0, 1.0, t),
            EasingFunction::EaseOut => cubic_bezier(0.0, 0.0, 0.58, 1.0, t),
            EasingFunction::EaseInOut => cubic_bezier(0.42, 0.0, 0.58, 1.0, t),
            EasingFunction::CubicBezier(x1, y1, x2, y2) => cubic_bezier(*x1, *y1, *x2, *y2, t),
            EasingFunction::Steps(n, timing) => {
                let n = (*n).max(1) as f32;
                match timing {
                    StepTiming::Start => (t * n).ceil() / n,
                    StepTiming::End => (t * n).floor() / n,
                }
            }
        }
    }
}

fn cubic_bezier(x1: f32, y1: f32, x2: f32, y2: f32, t: f32) -> f32 {
    let cx = 3.0 * x1;
    let bx = 3.0 * (x2 - x1) - cx;
    let ax = 1.0 - cx - bx;
    let cy = 3.0 * y1;
    let by = 3.0 * (y2 - y1) - cy;
    let ay = 1.0 - cy - by;

    let mut t_param = t;
    for _ in 0..8 {
        let x = ((ax * t_param + bx) * t_param + cx) * t_param - t;
        let dx = (3.0 * ax * t_param + 2.0 * bx) * t_param + cx;
        if dx.abs() < 1e-6 {
            break;
        }
        t_param -= x / dx;
    }
    t_param = t_param.clamp(0.0, 1.0);
    ((ay * t_param + by) * t_param + cy) * t_param
}

#[derive(Debug, Clone)]
pub struct TransitionState {
    pub properties: Vec<TransitionProperty>,
    pub elapsed_ms: f32,
    pub is_running: bool,
}

impl Default for TransitionState {
    fn default() -> Self {
        Self::new()
    }
}

impl TransitionState {
    pub fn new() -> Self {
        TransitionState {
            properties: Vec::new(),
            elapsed_ms: 0.0,
            is_running: false,
        }
    }

    pub fn add_property(&mut self, prop: TransitionProperty) {
        self.properties.push(prop);
    }

    pub fn start(&mut self) {
        self.is_running = true;
        self.elapsed_ms = 0.0;
    }

    pub fn stop(&mut self) {
        self.is_running = false;
    }

    pub fn tick(&mut self, delta_ms: f32) {
        if self.is_running {
            self.elapsed_ms += delta_ms;
        }
    }

    pub fn is_complete(&self) -> bool {
        if !self.is_running {
            return false;
        }
        self.properties
            .iter()
            .all(|p| self.elapsed_ms >= p.total_duration_ms())
    }

    pub fn progress(&self, property_name: &str) -> f32 {
        if let Some(prop) = self.properties.iter().find(|p| p.name == property_name) {
            prop.progress_at(self.elapsed_ms)
        } else {
            1.0
        }
    }

    pub fn progress_all(&self) -> Vec<(&str, f32)> {
        self.properties
            .iter()
            .map(|p| (p.name.as_str(), p.progress_at(self.elapsed_ms)))
            .collect()
    }

    pub fn interpolate_f32(&self, property_name: &str, from: f32, to: f32) -> f32 {
        let t = self.progress(property_name);
        from + (to - from) * t
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linear_easing() {
        let easing = EasingFunction::Linear;
        assert_eq!(easing.apply(0.0), 0.0);
        assert_eq!(easing.apply(0.5), 0.5);
        assert_eq!(easing.apply(1.0), 1.0);
    }

    #[test]
    fn test_steps_end() {
        let easing = EasingFunction::Steps(3, StepTiming::End);
        assert_eq!(easing.apply(0.0), 0.0);
        assert_eq!(easing.apply(0.3), 0.0);
        assert_eq!(easing.apply(0.4), 1.0 / 3.0);
        assert_eq!(easing.apply(0.7), 2.0 / 3.0);
        assert_eq!(easing.apply(1.0), 1.0);
    }

    #[test]
    fn test_steps_start() {
        let easing = EasingFunction::Steps(3, StepTiming::Start);
        assert_eq!(easing.apply(0.0), 1.0 / 3.0);
        assert_eq!(easing.apply(0.3), 1.0 / 3.0);
        assert_eq!(easing.apply(0.4), 2.0 / 3.0);
        assert_eq!(easing.apply(1.0), 1.0);
    }

    #[test]
    fn test_cubic_bezier_at_endpoints() {
        let result = cubic_bezier(0.25, 0.1, 0.25, 1.0, 0.0);
        assert!((result - 0.0).abs() < 0.01);
        let result = cubic_bezier(0.25, 0.1, 0.25, 1.0, 1.0);
        assert!((result - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_transition_property_progress() {
        let prop = TransitionProperty::new("opacity", 100.0);
        assert_eq!(prop.progress_at(0.0), 0.0);
        assert_eq!(prop.progress_at(50.0), 0.5);
        assert_eq!(prop.progress_at(100.0), 1.0);
        assert_eq!(prop.progress_at(200.0), 1.0);
    }

    #[test]
    fn test_transition_property_with_delay() {
        let prop = TransitionProperty::new("opacity", 100.0).with_delay(50.0);
        assert_eq!(prop.progress_at(0.0), 0.0);
        assert_eq!(prop.progress_at(49.0), 0.0);
        assert_eq!(prop.progress_at(50.0), 0.0);
        assert_eq!(prop.progress_at(100.0), 0.5);
        assert_eq!(prop.progress_at(150.0), 1.0);
    }

    #[test]
    fn test_total_duration() {
        let prop = TransitionProperty::new("opacity", 100.0).with_delay(50.0);
        assert_eq!(prop.total_duration_ms(), 150.0);
    }

    #[test]
    fn test_transition_state_tick() {
        let mut state = TransitionState::new();
        state.add_property(TransitionProperty::new("opacity", 200.0));
        state.start();
        state.tick(100.0);
        assert_eq!(state.progress("opacity"), 0.5);
        state.tick(100.0);
        assert_eq!(state.progress("opacity"), 1.0);
    }

    #[test]
    fn test_transition_state_complete() {
        let mut state = TransitionState::new();
        state.add_property(TransitionProperty::new("opacity", 100.0));
        state.start();
        assert!(!state.is_complete());
        state.tick(100.0);
        assert!(state.is_complete());
    }

    #[test]
    fn test_interpolate_f32() {
        let mut state = TransitionState::new();
        state.add_property(TransitionProperty::new("opacity", 100.0));
        state.start();
        state.tick(50.0);
        let value = state.interpolate_f32("opacity", 0.0, 100.0);
        assert_eq!(value, 50.0);
    }

    #[test]
    fn test_progress_all() {
        let mut state = TransitionState::new();
        state.add_property(TransitionProperty::new("opacity", 100.0));
        state.add_property(TransitionProperty::new("transform", 200.0));
        state.start();
        state.tick(50.0);
        let results = state.progress_all();
        assert_eq!(results.len(), 2);
    }
}
