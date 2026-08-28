#[derive(Debug, Clone, Copy)]
pub struct MouseEvent {
    pub x: f32,
    pub y: f32,
    pub button: MouseButton,
    pub action: MouseAction,
    pub modifiers: ModifierKeys,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
    Back,
    Forward,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MouseAction {
    Down,
    Up,
    Move,
    Click,
    DoubleClick,
    Drag,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct ModifierKeys {
    pub shift: bool,
    pub ctrl: bool,
    pub alt: bool,
    pub meta: bool,
}

impl MouseEvent {
    pub fn new(x: f32, y: f32, button: MouseButton, action: MouseAction) -> Self {
        Self {
            x,
            y,
            button,
            action,
            modifiers: ModifierKeys::default(),
        }
    }

    pub fn with_modifiers(mut self, modifiers: ModifierKeys) -> Self {
        self.modifiers = modifiers;
        self
    }
}
