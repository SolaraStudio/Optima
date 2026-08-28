pub struct KeyEvent {
    pub key: String,
    pub code: u16,
    pub action: KeyAction,
}

#[derive(Debug, Clone, Copy)]
pub enum KeyAction {
    Down,
    Up,
}

impl KeyEvent {
    pub fn new(key: &str, code: u16, action: KeyAction) -> Self {
        Self {
            key: key.to_string(),
            code,
            action,
        }
    }
}
