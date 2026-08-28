pub mod mouse;
pub mod keyboard;
pub mod touch;

use std::collections::VecDeque;

pub struct EventQueue {
    events: VecDeque<Event>,
}

#[derive(Debug, Clone)]
pub enum Event {
    Mouse(mouse::MouseEvent),
    Keyboard(keyboard::KeyEvent),
    Touch(touch::TouchEvent),
    Resize { width: u32, height: u32 },
    Scroll { delta_x: f32, delta_y: f32 },
}

impl EventQueue {
    pub fn new() -> Self {
        Self {
            events: VecDeque::new(),
        }
    }

    pub fn push(&mut self, event: Event) {
        self.events.push_back(event);
    }

    pub fn pop(&mut self) -> Option<Event> {
        self.events.pop_front()
    }

    pub fn is_empty(&self) -> bool {
        self.events.is_empty()
    }

    pub fn clear(&mut self) {
        self.events.clear();
    }
}
