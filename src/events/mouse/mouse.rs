#[derive(Debug, Clone)]
pub struct MouseEvent {
    pub x: f32,
    pub y: f32,
    pub client_x: f32,
    pub client_y: f32,
    pub screen_x: f32,
    pub screen_y: f32,
    pub button: u8,
    pub buttons: u8,
    pub click_count: u32,
    pub delta_x: f32,
    pub delta_y: f32,
    pub ctrl_key: bool,
    pub shift_key: bool,
    pub alt_key: bool,
    pub meta_key: bool,
}

impl MouseEvent {
    pub fn new(x: f32, y: f32, button: u8) -> Self {
        MouseEvent {
            x,
            y,
            client_x: x,
            client_y: y,
            screen_x: x,
            screen_y: y,
            button,
            buttons: 0,
            click_count: 1,
            delta_x: 0.0,
            delta_y: 0.0,
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

    pub fn with_delta(mut self, dx: f32, dy: f32) -> Self {
        self.delta_x = dx;
        self.delta_y = dy;
        self
    }

    pub fn with_click_count(mut self, count: u32) -> Self {
        self.click_count = count;
        self
    }

    pub fn with_buttons(mut self, buttons: u8) -> Self {
        self.buttons = buttons;
        self
    }

    pub fn is_left(&self) -> bool {
        self.button == 1
    }

    pub fn is_right(&self) -> bool {
        self.button == 2
    }

    pub fn is_middle(&self) -> bool {
        self.button == 3
    }
}
