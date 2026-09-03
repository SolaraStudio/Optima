use crate::events::touch::{TouchEvent, TouchPoint};

#[derive(Debug, Clone)]
pub struct TouchEndEvent {
    pub touches: Vec<TouchPoint>,
    pub changed_touches: Vec<TouchPoint>,
    pub target_touches: Vec<TouchPoint>,
    pub ctrl_key: bool,
    pub shift_key: bool,
    pub alt_key: bool,
    pub meta_key: bool,
}

impl Default for TouchEndEvent {
    fn default() -> Self {
        Self::new()
    }
}

impl TouchEndEvent {
    pub fn new() -> Self {
        TouchEndEvent {
            touches: Vec::new(),
            changed_touches: Vec::new(),
            target_touches: Vec::new(),
            ctrl_key: false,
            shift_key: false,
            alt_key: false,
            meta_key: false,
        }
    }

    pub fn from_touch_event(event: &TouchEvent) -> Self {
        TouchEndEvent {
            touches: event.touches.clone(),
            changed_touches: event.changed_touches.clone(),
            target_touches: event.target_touches.clone(),
            ctrl_key: event.ctrl_key,
            shift_key: event.shift_key,
            alt_key: event.alt_key,
            meta_key: event.meta_key,
        }
    }

    pub fn with_modifiers(mut self, ctrl: bool, shift: bool, alt: bool, meta: bool) -> Self {
        self.ctrl_key = ctrl;
        self.shift_key = shift;
        self.alt_key = alt;
        self.meta_key = meta;
        self
    }
}
