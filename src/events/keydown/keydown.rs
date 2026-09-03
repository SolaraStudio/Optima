use crate::events::keyboard::KeyEvent;

#[derive(Debug, Clone)]
pub struct KeyDownEvent {
    pub key: String,
    pub code: String,
    pub location: u32,
    pub repeat: bool,
    pub ctrl_key: bool,
    pub shift_key: bool,
    pub alt_key: bool,
    pub meta_key: bool,
    pub is_composing: bool,
    pub key_code: u16,
}

impl KeyDownEvent {
    pub fn new(key: &str, code: &str) -> Self {
        KeyDownEvent {
            key: key.to_string(),
            code: code.to_string(),
            location: 0,
            repeat: false,
            ctrl_key: false,
            shift_key: false,
            alt_key: false,
            meta_key: false,
            is_composing: false,
            key_code: 0,
        }
    }

    pub fn from_key_event(event: &KeyEvent) -> Self {
        KeyDownEvent {
            key: event.key.clone(),
            code: event.code.clone(),
            location: event.location,
            repeat: event.repeat,
            ctrl_key: event.ctrl_key,
            shift_key: event.shift_key,
            alt_key: event.alt_key,
            meta_key: event.meta_key,
            is_composing: event.is_composing,
            key_code: 0,
        }
    }

    pub fn with_key_code(mut self, key_code: u16) -> Self {
        self.key_code = key_code;
        self
    }

    pub fn with_modifiers(mut self, ctrl: bool, shift: bool, alt: bool, meta: bool) -> Self {
        self.ctrl_key = ctrl;
        self.shift_key = shift;
        self.alt_key = alt;
        self.meta_key = meta;
        self
    }
}
