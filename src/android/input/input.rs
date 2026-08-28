use crate::events::{EventQueue, Event};
use crate::events::touch::{TouchEvent, TouchPhase};
use crate::events::mouse::{MouseEvent, MouseButton, MouseAction};
use crate::events::keyboard::{KeyEvent, KeyAction};

pub struct AndroidInput {
    event_queue: EventQueue,
    pointer_down: bool,
    last_x: f32,
    last_y: f32,
}

impl AndroidInput {
    pub fn new() -> Self {
        Self {
            event_queue: EventQueue::new(),
            pointer_down: false,
            last_x: 0.0,
            last_y: 0.0,
        }
    }

    pub fn handle_touch(&mut self, x: f32, y: f32, action: u32) {
        let phase = match action {
            0 => TouchPhase::Down,
            1 => TouchPhase::Up,
            2 => TouchPhase::Move,
            3 => TouchPhase::Cancel,
            _ => TouchPhase::Cancel,
        };
        let event = Event::Touch(TouchEvent::new(x, y, 0, phase));
        self.event_queue.push(event);

        if phase == TouchPhase::Down {
            self.pointer_down = true;
            self.last_x = x;
            self.last_y = y;
        } else if phase == TouchPhase::Up || phase == TouchPhase::Cancel {
            self.pointer_down = false;
        } else if phase == TouchPhase::Move {
            self.last_x = x;
            self.last_y = y;
        }
    }

    pub fn handle_touch_with_id(&mut self, x: f32, y: f32, id: u32, action: u32) {
        let phase = match action {
            0 => TouchPhase::Down,
            1 => TouchPhase::Up,
            2 => TouchPhase::Move,
            3 => TouchPhase::Cancel,
            _ => TouchPhase::Cancel,
        };
        let event = Event::Touch(TouchEvent::new(x, y, id, phase));
        self.event_queue.push(event);

        if phase == TouchPhase::Down {
            self.pointer_down = true;
            self.last_x = x;
            self.last_y = y;
        } else if phase == TouchPhase::Up || phase == TouchPhase::Cancel {
            self.pointer_down = false;
        }
    }

    pub fn handle_mouse(&mut self, x: f32, y: f32, button: u32, action: u32) {
        let btn = match button {
            1 => MouseButton::Left,
            2 => MouseButton::Right,
            3 => MouseButton::Middle,
            4 => MouseButton::Back,
            5 => MouseButton::Forward,
            _ => MouseButton::Left,
        };
        let act = match action {
            0 => MouseAction::Down,
            1 => MouseAction::Up,
            2 => MouseAction::Move,
            3 => MouseAction::Click,
            _ => MouseAction::Move,
        };
        let event = Event::Mouse(MouseEvent::new(x, y, btn, act));
        self.event_queue.push(event);
    }

    pub fn handle_mouse_with_delta(&mut self, x: f32, y: f32, dx: f32, dy: f32, button: u32, action: u32) {
        let btn = match button {
            1 => MouseButton::Left,
            2 => MouseButton::Right,
            3 => MouseButton::Middle,
            _ => MouseButton::Left,
        };
        let act = match action {
            0 => MouseAction::Down,
            1 => MouseAction::Up,
            2 => MouseAction::Move,
            3 => MouseAction::Click,
            4 => MouseAction::Wheel,
            _ => MouseAction::Move,
        };
        let mut event = MouseEvent::new(x, y, btn, act);
        event.delta_x = dx;
        event.delta_y = dy;
        self.event_queue.push(Event::Mouse(event));
    }

    pub fn handle_key(&mut self, key_code: u16, key: &str, action: u32) {
        let act = match action {
            0 => KeyAction::Down,
            1 => KeyAction::Up,
            2 => KeyAction::Repeat,
            _ => KeyAction::Up,
        };
        let event = Event::Keyboard(KeyEvent::new(key, key_code, act));
        self.event_queue.push(event);
    }

    pub fn get_events(&mut self) -> Vec<Event> {
        let mut events = Vec::new();
        while let Some(event) = self.event_queue.pop() {
            events.push(event);
        }
        events
    }

    pub fn is_pointer_down(&self) -> bool {
        self.pointer_down
    }

    pub fn get_last_position(&self) -> (f32, f32) {
        (self.last_x, self.last_y)
    }

    pub fn clear(&mut self) {
        self.event_queue.clear();
        self.pointer_down = false;
    }

    pub fn push_event(&mut self, event: Event) {
        self.event_queue.push(event);
    }
}

impl Default for AndroidInput {
    fn default() -> Self {
        Self::new()
    }
}
