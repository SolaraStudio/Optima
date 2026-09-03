#[derive(Debug, Clone)]
pub struct TouchEvent {
    pub touches: Vec<TouchPoint>,
    pub changed_touches: Vec<TouchPoint>,
    pub target_touches: Vec<TouchPoint>,
    pub ctrl_key: bool,
    pub shift_key: bool,
    pub alt_key: bool,
    pub meta_key: bool,
}

#[derive(Debug, Clone)]
pub struct TouchPoint {
    pub identifier: u32,
    pub x: f32,
    pub y: f32,
    pub client_x: f32,
    pub client_y: f32,
    pub screen_x: f32,
    pub screen_y: f32,
    pub radius_x: f32,
    pub radius_y: f32,
    pub rotation_angle: f32,
    pub force: f32,
    pub target: Option<String>,
}

impl Default for TouchEvent {
    fn default() -> Self {
        Self::new()
    }
}

impl TouchEvent {
    pub fn new() -> Self {
        TouchEvent {
            touches: Vec::new(),
            changed_touches: Vec::new(),
            target_touches: Vec::new(),
            ctrl_key: false,
            shift_key: false,
            alt_key: false,
            meta_key: false,
        }
    }

    pub fn with_touches(mut self, touches: Vec<TouchPoint>) -> Self {
        self.touches = touches;
        self
    }

    pub fn with_changed_touches(mut self, touches: Vec<TouchPoint>) -> Self {
        self.changed_touches = touches;
        self
    }

    pub fn with_target_touches(mut self, touches: Vec<TouchPoint>) -> Self {
        self.target_touches = touches;
        self
    }

    pub fn with_modifiers(mut self, ctrl: bool, shift: bool, alt: bool, meta: bool) -> Self {
        self.ctrl_key = ctrl;
        self.shift_key = shift;
        self.alt_key = alt;
        self.meta_key = meta;
        self
    }

    pub fn touch_count(&self) -> usize {
        self.touches.len()
    }

    pub fn changed_touch_count(&self) -> usize {
        self.changed_touches.len()
    }
}

impl TouchPoint {
    pub fn new(identifier: u32, x: f32, y: f32) -> Self {
        TouchPoint {
            identifier,
            x,
            y,
            client_x: x,
            client_y: y,
            screen_x: x,
            screen_y: y,
            radius_x: 1.0,
            radius_y: 1.0,
            rotation_angle: 0.0,
            force: 1.0,
            target: None,
        }
    }

    pub fn with_radius(mut self, rx: f32, ry: f32) -> Self {
        self.radius_x = rx;
        self.radius_y = ry;
        self
    }

    pub fn with_force(mut self, force: f32) -> Self {
        self.force = force;
        self
    }

    pub fn with_rotation(mut self, angle: f32) -> Self {
        self.rotation_angle = angle;
        self
    }

    pub fn with_target(mut self, target: &str) -> Self {
        self.target = Some(target.to_string());
        self
    }
}
