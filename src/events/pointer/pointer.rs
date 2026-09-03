#[derive(Debug, Clone)]
pub struct PointerEvent {
    pub pointer_id: u32,
    pub pointer_type: PointerType,
    pub x: f32,
    pub y: f32,
    pub client_x: f32,
    pub client_y: f32,
    pub screen_x: f32,
    pub screen_y: f32,
    pub width: f32,
    pub height: f32,
    pub pressure: f32,
    pub tilt_x: f32,
    pub tilt_y: f32,
    pub is_primary: bool,
    pub button: u8,
    pub buttons: u8,
    pub ctrl_key: bool,
    pub shift_key: bool,
    pub alt_key: bool,
    pub meta_key: bool,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PointerType {
    Mouse,
    Pen,
    Touch,
}

impl PointerEvent {
    pub fn new(pointer_id: u32, pointer_type: PointerType, x: f32, y: f32) -> Self {
        PointerEvent {
            pointer_id,
            pointer_type,
            x,
            y,
            client_x: x,
            client_y: y,
            screen_x: x,
            screen_y: y,
            width: 1.0,
            height: 1.0,
            pressure: 1.0,
            tilt_x: 0.0,
            tilt_y: 0.0,
            is_primary: true,
            button: 0,
            buttons: 0,
            ctrl_key: false,
            shift_key: false,
            alt_key: false,
            meta_key: false,
        }
    }

    pub fn with_modifiers(mut self, ctrl: bool, shift: bool, alt: bool, meta: bool) -> Self {
        self.ctrl_key = ctrl;
        self.shift_key = shift;
        self.alt_key = alt;
        self.meta_key = meta;
        self
    }

    pub fn with_pressure(mut self, pressure: f32) -> Self {
        self.pressure = pressure;
        self
    }

    pub fn with_size(mut self, width: f32, height: f32) -> Self {
        self.width = width;
        self.height = height;
        self
    }

    pub fn with_tilt(mut self, tilt_x: f32, tilt_y: f32) -> Self {
        self.tilt_x = tilt_x;
        self.tilt_y = tilt_y;
        self
    }

    pub fn is_mouse(&self) -> bool {
        self.pointer_type == PointerType::Mouse
    }

    pub fn is_pen(&self) -> bool {
        self.pointer_type == PointerType::Pen
    }

    pub fn is_touch(&self) -> bool {
        self.pointer_type == PointerType::Touch
    }
}
