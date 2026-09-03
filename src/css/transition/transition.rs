use crate::css::value::Value;

#[derive(Debug, Clone)]
pub struct Transition {
    pub property: String,
    pub duration: f32,
    pub timing_function: String,
    pub delay: f32,
}

impl Transition {
    pub fn new(property: &str, duration: f32, timing_function: &str, delay: f32) -> Self {
        Transition {
            property: property.to_string(),
            duration,
            timing_function: timing_function.to_string(),
            delay,
        }
    }

    pub fn from_value(_value: &Value) -> Option<Self> {
        // Simplified: expects a list of values
        None
    }
}
