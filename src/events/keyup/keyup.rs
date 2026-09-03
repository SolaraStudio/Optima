use crate::events::keyboard::KeyEvent;

#[derive(Debug, Clone)]
pub struct KeyUpEvent {
    pub key: String,
    pub code: String,
    pub location: u32,
    pub repeat: bool,
    pub ctrl_key: bool,
    pub shift_key: bool,
    pub alt_key: bool,
    pub meta_key: bool,
    pub is_composing: bool,
}

impl KeyUpEvent {
    pub fn new(key: &str, code: &str) -> Self {
        KeyUpEvent {
            key: key.to_string(),
            code: code.to_string(),
            location: 0,
            repeat: false,
            ctrl_key: false,
            shift_key: false,
            alt_key: false,
            meta_key: false,
            is_composing: false,
        }
    }

    pub fn from_key_event(event: &KeyEvent) -> Self {
        KeyUpEvent {
            key: event.key.clone(),
            code: event.code.clone(),
            location: event.location,
            repeat: event.repeat,
            ctrl_key: event.ctrl_key,
            shift_key: event.shift_key,
            alt_key: event.alt_key,
            meta_key: event.meta_key,
            is_composing: event.is_composing,
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
