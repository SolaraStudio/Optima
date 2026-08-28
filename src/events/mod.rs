pub mod mouse;
pub mod keyboard;
pub mod touch;
pub mod gesture;
pub mod focus;
pub mod scroll;
pub mod resize;
pub mod click;
pub mod pointer;
pub mod mousedown;
pub mod mouseup;
pub mod mousemove;
pub mod keydown;
pub mod keyup;

use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub enum Event {
    Mouse(mouse::MouseEvent),
    Keyboard(keyboard::KeyEvent),
    Touch(touch::TouchEvent),
    Gesture(gesture::GestureEvent),
    Focus(focus::FocusEvent),
    Scroll(scroll::ScrollEvent),
    Resize(resize::ResizeEvent),
    Click(click::ClickEvent),
    Pointer(pointer::PointerEvent),
}

pub struct EventQueue {
    events: VecDeque<Event>,
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

    pub fn peek(&self) -> Option<&Event> {
        self.events.front()
    }

    pub fn is_empty(&self) -> bool {
        self.events.is_empty()
    }

    pub fn len(&self) -> usize {
        self.events.len()
    }

    pub fn clear(&mut self) {
        self.events.clear();
    }

    pub fn extend(&mut self, other: &mut EventQueue) {
        while let Some(event) = other.pop() {
            self.push(event);
        }
    }

    pub fn filter<F>(&mut self, predicate: F)
    where
        F: Fn(&Event) -> bool,
    {
        self.events.retain(predicate);
    }
}

impl Default for EventQueue {
    fn default() -> Self {
        Self::new()
    }
}
