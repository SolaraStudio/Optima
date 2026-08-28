pub struct MouseEvent {
    pub x: f32,
    pub y: f32,
    pub button: u8,
    pub action: MouseAction,
}

#[derive(Debug, Clone, Copy)]
pub enum MouseAction {
    Down,
    Up,
    Move,
    Click,
}

impl MouseEvent {
    pub fn new(x: f32, y: f32, button: u8, action: MouseAction) -> Self {
        Self { x, y, button, action }
    }
}
