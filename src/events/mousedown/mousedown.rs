use crate::events::mouse::MouseEvent;

#[derive(Debug, Clone)]
pub struct MouseDownEvent {
    pub x: f32,
    pub y: f32,
    pub client_x: f32,
    pub client_y: f32,
    pub screen_x: f32,
    pub screen_y: f32,
    pub button: u8,
    pub buttons: u8,
    pub click_count: u32,
    pub ctrl_key: bool,
    pub shift_key: bool,
    pub alt_key: bool,
    pub meta_key: bool,
}

impl MouseDownEvent {
    pub fn new(x: f32, y: f32, button: u8) -> Self {
        MouseDownEvent {
            x,
            y,
            client_x: x,
            client_y: y,
            screen_x: x,
            screen_y: y,
            button,
            buttons: 0,
            click_count: 1,
            ctrl_key: false,
            shift_key: false,
            alt_key: false,
            meta_key: false,
        }
    }

    pub fn from_mouse_event(event: &MouseEvent) -> Self {
        MouseDownEvent {
            x: event.x,
            y: event.y,
            client_x: event.client_x,
            client_y: event.client_y,
            screen_x: event.screen_x,
            screen_y: event.screen_y,
            button: event.button,
            buttons: event.buttons,
            click_count: event.click_count,
            ctrl_key: event.ctrl_key,
            shift_key: event.shift_key,
            alt_key: event.alt_key,
            meta_key: event.meta_key,
        }
    }

    pub fn with_modifiers(mut self, ctrl: bool, shift: bool, alt: bool, meta: bool) -> Self {
        self.ctrl_key = ctrl;
        self.shift_key = shift;
        self.alt_key = alt;
        self.meta_key = meta;
        self
    }
}
