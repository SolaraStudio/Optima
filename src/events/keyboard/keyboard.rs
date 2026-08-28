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

impl ModifierKeys {
    pub fn is_pressed(&self) -> bool {
        self.shift || self.ctrl || self.alt || self.meta
    }
}

#[derive(Debug, Clone)]
pub struct KeyEvent {
    pub key: String,
    pub code: u16,
    pub action: KeyAction,
    pub modifiers: ModifierKeys,
    pub repeat_count: u32,
    pub key_code: u16,
    pub char_code: Option<char>,
}

impl KeyEvent {
    pub fn new(key: &str, code: u16, action: KeyAction) -> Self {
        Self {
            key: key.to_string(),
            code,
            action,
            modifiers: ModifierKeys::default(),
            repeat_count: 1,
            key_code: code,
            char_code: key.chars().next(),
        }
    }

    pub fn with_modifiers(mut self, modifiers: ModifierKeys) -> Self {
        self.modifiers = modifiers;
        self
    }

    pub fn with_repeat_count(mut self, count: u32) -> Self {
        self.repeat_count = count;
        self
    }

    pub fn with_key_code(mut self, key_code: u16) -> Self {
        self.key_code = key_code;
        self
    }

    pub fn is_character(&self) -> bool {
        self.key.len() == 1 && self.key.chars().next().unwrap().is_alphanumeric()
    }

    pub fn is_control(&self) -> bool {
        matches!(self.key.as_str(), "Escape" | "Enter" | "Backspace" | "Tab" | "Delete" | "ArrowUp" | "ArrowDown" | "ArrowLeft" | "ArrowRight")
    }

    pub fn is_function(&self) -> bool {
        self.key.starts_with("F") && self.key.len() <= 3 && self.key[1..].parse::<u32>().is_ok()
    }

    pub fn is_down(&self) -> bool {
        matches!(self.action, KeyAction::Down)
    }

    pub fn is_up(&self) -> bool {
        matches!(self.action, KeyAction::Up)
    }

    pub fn is_repeat(&self) -> bool {
        matches!(self.action, KeyAction::Repeat)
    }

    pub fn is_enter(&self) -> bool {
        self.key == "Enter" || self.key == "\r"
    }

    pub fn is_escape(&self) -> bool {
        self.key == "Escape"
    }

    pub fn is_backspace(&self) -> bool {
        self.key == "Backspace"
    }

    pub fn is_tab(&self) -> bool {
        self.key == "Tab"
    }

    pub fn is_arrow(&self) -> bool {
        self.key.starts_with("Arrow")
    }
}
