use crate::events::keyboard::KeyEvent;
use crate::events::mouse::MouseEvent;
use crate::events::touch::TouchEvent;
use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub enum InputEvent {
    Touch(TouchEvent),
    Mouse(MouseEvent),
    Key(KeyEvent),
}

#[derive(Debug, Clone)]
pub struct EventQueue {
    pub events: VecDeque<InputEvent>,
}

impl Default for EventQueue {
    fn default() -> Self {
        Self::new()
    }
}

impl EventQueue {
    pub fn new() -> Self {
        EventQueue {
            events: VecDeque::new(),
        }
    }

    pub fn push(&mut self, event: InputEvent) {
        self.events.push_back(event);
    }

    pub fn pop(&mut self) -> Option<InputEvent> {
        self.events.pop_front()
    }

    pub fn clear(&mut self) {
        self.events.clear();
    }

    pub fn is_empty(&self) -> bool {
        self.events.is_empty()
    }
}

pub struct AndroidInput {
    pub event_queue: EventQueue,
    pointer_down: bool,
    last_x: f32,
    last_y: f32,
}

impl Default for AndroidInput {
    fn default() -> Self {
        Self::new()
    }
}

impl AndroidInput {
    pub fn new() -> Self {
        AndroidInput {
            event_queue: EventQueue::new(),
            pointer_down: false,
            last_x: 0.0,
            last_y: 0.0,
        }
    }

    pub fn handle_touch(&mut self, x: f32, y: f32, action: u32) {
        let mut event = TouchEvent::new();
        event = match action {
            0 => event.with_changed_touches(vec![crate::events::touch::TouchPoint::new(0, x, y)]),
            2 => event.with_changed_touches(vec![crate::events::touch::TouchPoint::new(0, x, y)]),
            _ => event,
        };
        self.event_queue.push(InputEvent::Touch(event));
        match action {
            0 => {
                self.pointer_down = true;
                self.last_x = x;
                self.last_y = y;
            }
            1 | 3 => {
                self.pointer_down = false;
            }
            _ => {}
        }
    }

    pub fn handle_mouse(&mut self, x: f32, y: f32, button: u32, action: u32) {
        let btn = match button {
            2 => 2,
            3 => 3,
            _ => 1,
        };
        let _act = action;
        let event = MouseEvent::new(x, y, btn);
        self.event_queue.push(InputEvent::Mouse(event));
    }

    pub fn handle_key(&mut self, key_code: u16, key: &str, action: u32) {
        let _code = key_code;
        let _act = action;
        let event = KeyEvent::new(key, &key_code.to_string());
        self.event_queue.push(InputEvent::Key(event));
    }

    pub fn get_events(&mut self) -> Vec<InputEvent> {
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
}
