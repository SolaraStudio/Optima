#[derive(Debug, Clone)]
pub struct TouchEvent {
    pub x: f32,
    pub y: f32,
    pub id: u32,
    pub phase: TouchPhase,
    pub radius_x: f32,
    pub radius_y: f32,
    pub pressure: f32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TouchPhase {
    Down,
    Move,
    Up,
    Cancel,
}

impl TouchEvent {
    pub fn new(x: f32, y: f32, id: u32, phase: TouchPhase) -> Self {
        Self {
            x,
            y,
            id,
            phase,
            radius_x: 1.0,
            radius_y: 1.0,
            pressure: 1.0,
        }
    }

    pub fn with_radius(mut self, rx: f32, ry: f32) -> Self {
        self.radius_x = rx;
        self.radius_y = ry;
        self
    }

    pub fn with_pressure(mut self, pressure: f32) -> Self {
        self.pressure = pressure;
        self
    }
}
