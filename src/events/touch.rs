pub struct TouchEvent {
    pub x: f32,
    pub y: f32,
    pub id: u32,
    pub phase: TouchPhase,
}

#[derive(Debug, Clone, Copy)]
pub enum TouchPhase {
    Down,
    Move,
    Up,
    Cancel,
}

impl TouchEvent {
    pub fn new(x: f32, y: f32, id: u32, phase: TouchPhase) -> Self {
        Self { x, y, id, phase }
    }
}
