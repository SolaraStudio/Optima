#[derive(Debug, Clone)]
pub struct KeyEvent {
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

impl KeyEvent {
    pub fn new(key: &str, code: &str) -> Self {
        KeyEvent {
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

    pub fn with_modifiers(mut self, ctrl: bool, shift: bool, alt: bool, meta: bool) -> Self {
        self.ctrl_key = ctrl;
        self.shift_key = shift;
        self.alt_key = alt;
        self.meta_key = meta;
        self
    }

    pub fn with_repeat(mut self, repeat: bool) -> Self {
        self.repeat = repeat;
        self
    }

    pub fn is_character(&self) -> bool {
        self.key.len() == 1 && self.key.chars().next().unwrap().is_alphanumeric()
    }

    pub fn is_control(&self) -> bool {
        matches!(
            self.key.as_str(),
            "Escape" | "Enter" | "Backspace" | "Tab" | "Delete"
        )
    }

    pub fn is_arrow(&self) -> bool {
        self.key.starts_with("Arrow")
    }

    pub fn is_function(&self) -> bool {
        self.key.starts_with('F') && self.key.len() <= 3 && self.key[1..].parse::<u32>().is_ok()
    }
}
