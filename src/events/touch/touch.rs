#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TouchPhase {
    Down,
    Move,
    Up,
    Cancel,
    Stationary,
}

#[derive(Debug, Clone)]
pub struct TouchEvent {
    pub x: f32,
    pub y: f32,
    pub client_x: f32,
    pub client_y: f32,
    pub screen_x: f32,
    pub screen_y: f32,
    pub id: u32,
    pub phase: TouchPhase,
    pub radius_x: f32,
    pub radius_y: f32,
    pub rotation: f32,
    pub pressure: f32,
    pub force: f32,
}

impl TouchEvent {
    pub fn new(x: f32, y: f32, id: u32, phase: TouchPhase) -> Self {
        Self {
            x,
            y,
            client_x: x,
            client_y: y,
            screen_x: x,
            screen_y: y,
            id,
            phase,
            radius_x: 1.0,
            radius_y: 1.0,
            rotation: 0.0,
            pressure: 1.0,
            force: 1.0,
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

    pub fn with_force(mut self, force: f32) -> Self {
        self.force = force;
        self
    }

    pub fn with_rotation(mut self, rotation: f32) -> Self {
        self.rotation = rotation;
        self
    }

    pub fn with_client(mut self, client_x: f32, client_y: f32) -> Self {
        self.client_x = client_x;
        self.client_y = client_y;
        self
    }

    pub fn with_screen(mut self, screen_x: f32, screen_y: f32) -> Self {
        self.screen_x = screen_x;
        self.screen_y = screen_y;
        self
    }

    pub fn is_down(&self) -> bool {
        matches!(self.phase, TouchPhase::Down)
    }

    pub fn is_move(&self) -> bool {
        matches!(self.phase, TouchPhase::Move)
    }

    pub fn is_up(&self) -> bool {
        matches!(self.phase, TouchPhase::Up)
    }

    pub fn is_cancel(&self) -> bool {
        matches!(self.phase, TouchPhase::Cancel)
    }

    pub fn is_stationary(&self) -> bool {
        matches!(self.phase, TouchPhase::Stationary)
    }
}
