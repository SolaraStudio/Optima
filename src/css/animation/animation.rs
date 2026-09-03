use crate::css::keyframes::Keyframes;

#[derive(Debug, Clone)]
pub struct Animation {
    pub name: String,
    pub duration: f32,
    pub timing_function: String,
    pub delay: f32,
    pub iteration_count: f32,
    pub direction: String,
    pub fill_mode: String,
    pub play_state: String,
    pub keyframes: Option<Keyframes>,
}

impl Animation {
    pub fn new(name: &str, duration: f32) -> Self {
        Animation {
            name: name.to_string(),
            duration,
            timing_function: "ease".to_string(),
            delay: 0.0,
            iteration_count: 1.0,
            direction: "normal".to_string(),
            fill_mode: "none".to_string(),
            play_state: "running".to_string(),
            keyframes: None,
        }
    }

    pub fn with_keyframes(mut self, keyframes: Keyframes) -> Self {
        self.keyframes = Some(keyframes);
        self
    }

    pub fn get_interpolated_value(
        &self,
        progress: f32,
    ) -> Option<Vec<crate::css::declaration::Declaration>> {
        self.keyframes.as_ref().map(|keyframes| keyframes.get_declarations_at(progress))
    }
}
