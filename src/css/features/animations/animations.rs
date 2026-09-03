use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum AnimationPlayState {
    Running,
    Paused,
    Idle,
    Finished,
}

#[derive(Debug, Clone, PartialEq)]
pub enum AnimationDirection {
    Normal,
    Reverse,
    Alternate,
    AlternateReverse,
}

#[derive(Debug, Clone, PartialEq)]
pub enum AnimationFillMode {
    None,
    Forwards,
    Backwards,
    Both,
}

#[derive(Debug, Clone, PartialEq)]
pub enum AnimationTimingFunction {
    Linear,
    Ease,
    EaseIn,
    EaseOut,
    EaseInOut,
    CubicBezier(f32, f32, f32, f32),
}

impl AnimationTimingFunction {
    pub fn apply(&self, t: f32) -> f32 {
        let t = t.clamp(0.0, 1.0);
        match self {
            AnimationTimingFunction::Linear => t,
            AnimationTimingFunction::Ease => {
                let c4 = (2.0 * std::f32::consts::PI) / 3.0;
                if t < 0.5 {
                    (2.0 * t * t * (1.0 + (c4 * t).sin())) / 1.0
                } else {
                    (2.0 - (2.0 * t - 2.0).powi(2) * ((c4 * t).sin() + 1.0)) / 2.0
                }
            }
            AnimationTimingFunction::EaseIn => t * t * t,
            AnimationTimingFunction::EaseOut => 1.0 - (1.0 - t).powi(3),
            AnimationTimingFunction::EaseInOut => {
                if t < 0.5 {
                    4.0 * t * t * t
                } else {
                    1.0 - (-2.0 * t + 2.0).powi(3) / 2.0
                }
            }
            AnimationTimingFunction::CubicBezier(_, _, _, _) => t,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Keyframe {
    pub offset: f32,
    pub properties: HashMap<String, String>,
    pub timing: Option<AnimationTimingFunction>,
}

impl Keyframe {
    pub fn new(offset: f32) -> Self {
        Keyframe {
            offset: offset.clamp(0.0, 1.0),
            properties: HashMap::new(),
            timing: None,
        }
    }

    pub fn set_property(&mut self, name: &str, value: &str) {
        self.properties.insert(name.to_string(), value.to_string());
    }

    pub fn get_property(&self, name: &str) -> Option<&str> {
        self.properties.get(name).map(|s| s.as_str())
    }
}

#[derive(Debug, Clone)]
pub struct KeyframesAnimation {
    pub name: String,
    pub keyframes: Vec<Keyframe>,
    pub duration_ms: f32,
    pub delay_ms: f32,
    pub iteration_count: f32,
    pub direction: AnimationDirection,
    pub fill_mode: AnimationFillMode,
    pub timing: AnimationTimingFunction,
    pub play_state: AnimationPlayState,
}

impl KeyframesAnimation {
    pub fn new(name: &str, duration_ms: f32) -> Self {
        KeyframesAnimation {
            name: name.to_string(),
            keyframes: Vec::new(),
            duration_ms,
            delay_ms: 0.0,
            iteration_count: 1.0,
            direction: AnimationDirection::Normal,
            fill_mode: AnimationFillMode::None,
            timing: AnimationTimingFunction::Ease,
            play_state: AnimationPlayState::Idle,
        }
    }

    pub fn add_keyframe(&mut self, keyframe: Keyframe) {
        self.keyframes.push(keyframe);
        self.keyframes
            .sort_by(|a, b| a.offset.partial_cmp(&b.offset).unwrap());
    }

    pub fn start(&mut self) {
        self.play_state = AnimationPlayState::Running;
    }

    pub fn pause(&mut self) {
        if self.play_state == AnimationPlayState::Running {
            self.play_state = AnimationPlayState::Paused;
        }
    }

    pub fn resume(&mut self) {
        if self.play_state == AnimationPlayState::Paused {
            self.play_state = AnimationPlayState::Running;
        }
    }

    pub fn reset(&mut self) {
        self.play_state = AnimationPlayState::Idle;
    }

    pub fn compute_progress(&self, elapsed_ms: f32) -> f32 {
        if self.duration_ms <= 0.0 {
            return 1.0;
        }
        let active = if elapsed_ms > self.delay_ms {
            elapsed_ms - self.delay_ms
        } else {
            return 0.0;
        };
        let progress = active / self.duration_ms;
        let iteration = progress.floor();
        let mut frac = progress - iteration;
        if self.iteration_count != f32::INFINITY && iteration >= self.iteration_count {
            return self.finish_progress();
        }
        match self.direction {
            AnimationDirection::Normal => {}
            AnimationDirection::Reverse => frac = 1.0 - frac,
            AnimationDirection::Alternate => {
                if (iteration as u32) % 2 == 1 {
                    frac = 1.0 - frac;
                }
            }
            AnimationDirection::AlternateReverse => {
                if (iteration as u32).is_multiple_of(2) {
                    frac = 1.0 - frac;
                }
            }
        }
        frac
    }

    fn finish_progress(&self) -> f32 {
        match self.fill_mode {
            AnimationFillMode::Forwards => {
                if let Some(last) = self.keyframes.last() {
                    last.offset
                } else {
                    1.0
                }
            }
            AnimationFillMode::Backwards => 0.0,
            AnimationFillMode::Both => 1.0,
            AnimationFillMode::None => 0.0,
        }
    }

    pub fn interpolate_properties(&self, progress: f32) -> HashMap<String, String> {
        let mut result = HashMap::new();
        if self.keyframes.is_empty() {
            return result;
        }
        let (prev, next) = find_bracketing_keyframes(&self.keyframes, progress);
        if prev == next {
            if let Some(kf) = self.keyframes.get(prev) {
                for (k, v) in &kf.properties {
                    result.insert(k.clone(), v.clone());
                }
            }
            return result;
        }
        let prev_kf = &self.keyframes[prev];
        let next_kf = &self.keyframes[next];
        let range = next_kf.offset - prev_kf.offset;
        let local_t = if range > 0.0 {
            (progress - prev_kf.offset) / range
        } else {
            0.0
        };
        let timing = next_kf.timing.as_ref().unwrap_or(&self.timing);
        let t = timing.apply(local_t);
        let mut all_keys: Vec<&String> = prev_kf
            .properties
            .keys()
            .chain(next_kf.properties.keys())
            .collect();
        all_keys.sort();
        all_keys.dedup();
        for key in all_keys {
            if let Some(v1) = prev_kf.properties.get(key) {
                if let Some(v2) = next_kf.properties.get(key) {
                    let interpolated = interpolate_value(v1, v2, t);
                    result.insert(key.clone(), interpolated);
                } else {
                    result.insert(key.clone(), v1.clone());
                }
            } else if let Some(v2) = next_kf.properties.get(key) {
                result.insert(key.clone(), v2.clone());
            }
        }
        result
    }
}

fn find_bracketing_keyframes(keyframes: &[Keyframe], progress: f32) -> (usize, usize) {
    if keyframes.len() <= 1 {
        return (0, 0);
    }
    let mut prev = 0;
    for (i, kf) in keyframes.iter().enumerate() {
        if kf.offset > progress {
            return (prev, i);
        }
        prev = i;
    }
    (prev, prev)
}

fn interpolate_value(from: &str, to: &str, t: f32) -> String {
    if let (Ok(from_num), Ok(to_num)) = (from.parse::<f32>(), to.parse::<f32>()) {
        let result = from_num + (to_num - from_num) * t;
        if result.fract() == 0.0 {
            format!("{}", result as i32)
        } else {
            format!("{:.2}", result)
        }
    } else {
        if t < 0.5 {
            from.to_string()
        } else {
            to.to_string()
        }
    }
}

#[derive(Debug, Clone)]
pub struct AnimationController {
    animations: Vec<KeyframesAnimation>,
    elapsed_ms: f32,
}

impl Default for AnimationController {
    fn default() -> Self {
        Self::new()
    }
}

impl AnimationController {
    pub fn new() -> Self {
        AnimationController {
            animations: Vec::new(),
            elapsed_ms: 0.0,
        }
    }

    pub fn add(&mut self, animation: KeyframesAnimation) {
        self.animations.push(animation);
    }

    pub fn remove(&mut self, name: &str) -> bool {
        let len_before = self.animations.len();
        self.animations.retain(|a| a.name != name);
        self.animations.len() < len_before
    }

    pub fn tick(&mut self, delta_ms: f32) {
        self.elapsed_ms += delta_ms;
        for anim in &mut self.animations {
            if anim.play_state == AnimationPlayState::Running {
                let _progress = anim.compute_progress(self.elapsed_ms - anim.delay_ms);
                if anim.iteration_count != f32::INFINITY {
                    let total = anim.duration_ms * anim.iteration_count + anim.delay_ms;
                    if self.elapsed_ms >= total {
                        anim.play_state = AnimationPlayState::Finished;
                    }
                }
            }
        }
    }

    pub fn state(&self, name: &str) -> Option<&AnimationPlayState> {
        self.animations
            .iter()
            .find(|a| a.name == name)
            .map(|a| &a.play_state)
    }

    pub fn properties(&self, name: &str) -> HashMap<String, String> {
        if let Some(anim) = self.animations.iter().find(|a| a.name == name) {
            let progress = anim.compute_progress(self.elapsed_ms - anim.delay_ms);
            anim.interpolate_properties(progress)
        } else {
            HashMap::new()
        }
    }

    pub fn running_count(&self) -> usize {
        self.animations
            .iter()
            .filter(|a| a.play_state == AnimationPlayState::Running)
            .count()
    }

    pub fn all_properties(&self) -> Vec<(&str, HashMap<String, String>)> {
        self.animations
            .iter()
            .map(|a| {
                let progress = a.compute_progress(self.elapsed_ms - a.delay_ms);
                (a.name.as_str(), a.interpolate_properties(progress))
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keyframe_set_get_property() {
        let mut kf = Keyframe::new(0.5);
        kf.set_property("opacity", "0.5");
        assert_eq!(kf.get_property("opacity"), Some("0.5"));
        assert_eq!(kf.get_property("missing"), None);
    }

    #[test]
    fn test_keyframe_offset_clamped() {
        let kf = Keyframe::new(2.0);
        assert_eq!(kf.offset, 1.0);
        let kf = Keyframe::new(-1.0);
        assert_eq!(kf.offset, 0.0);
    }

    #[test]
    fn test_animation_start_pause_resume() {
        let mut anim = KeyframesAnimation::new("fade", 1000.0);
        assert_eq!(anim.play_state, AnimationPlayState::Idle);
        anim.start();
        assert_eq!(anim.play_state, AnimationPlayState::Running);
        anim.pause();
        assert_eq!(anim.play_state, AnimationPlayState::Paused);
        anim.resume();
        assert_eq!(anim.play_state, AnimationPlayState::Running);
    }

    #[test]
    fn test_animation_progress_normal() {
        let mut anim = KeyframesAnimation::new("test", 100.0);
        anim.direction = AnimationDirection::Normal;
        assert_eq!(anim.compute_progress(0.0), 0.0);
        assert_eq!(anim.compute_progress(50.0), 0.5);
        assert_eq!(anim.compute_progress(100.0), 1.0);
    }

    #[test]
    fn test_animation_progress_reverse() {
        let mut anim = KeyframesAnimation::new("test", 100.0);
        anim.direction = AnimationDirection::Reverse;
        assert_eq!(anim.compute_progress(50.0), 0.5);
        assert_eq!(anim.compute_progress(100.0), 0.0);
    }

    #[test]
    fn test_animation_delay() {
        let mut anim = KeyframesAnimation::new("test", 100.0);
        anim.delay_ms = 50.0;
        assert_eq!(anim.compute_progress(0.0), 0.0);
        assert_eq!(anim.compute_progress(49.0), 0.0);
        assert_eq!(anim.compute_progress(50.0), 0.0);
        assert_eq!(anim.compute_progress(100.0), 0.5);
    }

    #[test]
    fn test_interpolate_properties() {
        let mut anim = KeyframesAnimation::new("test", 100.0);
        let mut kf0 = Keyframe::new(0.0);
        kf0.set_property("opacity", "0");
        let mut kf1 = Keyframe::new(1.0);
        kf1.set_property("opacity", "1");
        anim.add_keyframe(kf0);
        anim.add_keyframe(kf1);
        let props = anim.interpolate_properties(0.5);
        assert_eq!(props.get("opacity"), Some(&"0.50".to_string()));
    }

    #[test]
    fn test_timing_linear() {
        let timing = AnimationTimingFunction::Linear;
        assert_eq!(timing.apply(0.0), 0.0);
        assert_eq!(timing.apply(0.5), 0.5);
        assert_eq!(timing.apply(1.0), 1.0);
    }

    #[test]
    fn test_timing_ease_in() {
        let timing = AnimationTimingFunction::EaseIn;
        let result = timing.apply(0.5);
        assert!((result - 0.125).abs() < 0.01);
    }

    #[test]
    fn test_timing_ease_out() {
        let timing = AnimationTimingFunction::EaseOut;
        let result = timing.apply(0.5);
        assert!((result - 0.875).abs() < 0.01);
    }

    #[test]
    fn test_controller_tick() {
        let mut ctrl = AnimationController::new();
        let mut anim = KeyframesAnimation::new("fade", 100.0);
        anim.start();
        ctrl.add(anim);
        ctrl.tick(50.0);
        assert_eq!(ctrl.state("fade"), Some(&AnimationPlayState::Running));
        ctrl.tick(50.0);
        assert_eq!(ctrl.state("fade"), Some(&AnimationPlayState::Finished));
    }

    #[test]
    fn test_controller_remove() {
        let mut ctrl = AnimationController::new();
        ctrl.add(KeyframesAnimation::new("fade", 100.0));
        assert!(ctrl.remove("fade"));
        assert!(!ctrl.remove("fade"));
    }

    #[test]
    fn test_controller_properties() {
        let mut ctrl = AnimationController::new();
        let mut anim = KeyframesAnimation::new("test", 100.0);
        let mut kf0 = Keyframe::new(0.0);
        kf0.set_property("width", "0");
        let mut kf1 = Keyframe::new(1.0);
        kf1.set_property("width", "100");
        anim.add_keyframe(kf0);
        anim.add_keyframe(kf1);
        anim.start();
        ctrl.add(anim);
        ctrl.tick(50.0);
        let props = ctrl.properties("test");
        assert_eq!(props.get("width"), Some(&"50.00".to_string()));
    }

    #[test]
    fn test_running_count() {
        let mut ctrl = AnimationController::new();
        let mut a1 = KeyframesAnimation::new("a", 100.0);
        a1.start();
        let mut a2 = KeyframesAnimation::new("b", 100.0);
        a2.start();
        ctrl.add(a1);
        ctrl.add(a2);
        assert_eq!(ctrl.running_count(), 2);
        ctrl.tick(50.0);
        assert_eq!(ctrl.running_count(), 2);
        ctrl.tick(50.0);
        assert_eq!(ctrl.running_count(), 0);
    }
}
