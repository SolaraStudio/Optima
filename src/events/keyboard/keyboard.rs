#[derive(Debug, Clone)]
pub struct KeyEvent {
    pub key: String,
    pub code: u16,
    pub action: KeyAction,
    pub modifiers: ModifierKeys,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum KeyAction {
    Down,
    Up,
    Repeat,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct ModifierKeys {
    pub shift: bool,
    pub ctrl: bool,
    pub alt: bool,
    pub meta: bool,
}

impl KeyEvent {
    pub fn new(key: &str, code: u16, action: KeyAction) -> Self {
        Self {
            key: key.to_string(),
            code,
            action,
            modifiers: ModifierKeys::default(),
        }
    }

    pub fn with_modifiers(mut self, modifiers: ModifierKeys) -> Self {
        self.modifiers = modifiers;
        self
    }

    pub fn is_character(&self) -> bool {
        self.key.len() == 1 && self.key.chars().next().unwrap().is_alphanumeric()
    }

    pub fn is_control(&self) -> bool {
        matches!(self.key.as_str(), "Escape" | "Enter" | "Backspace" | "Tab" | "Delete")
    }
}
