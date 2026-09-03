use std::collections::HashMap;
use std::rc::Rc;

pub struct Event {
    pub type_: String,
    pub target: Option<EventTarget>,
    pub current_target: Option<EventTarget>,
    pub bubbles: bool,
    pub cancelable: bool,
    pub default_prevented: bool,
    pub propagation_stopped: bool,
}

#[derive(Clone)]
pub struct EventTarget {
    pub node: Option<usize>,
}

impl Event {
    pub fn new(type_: &str) -> Self {
        Event {
            type_: type_.to_string(),
            target: None,
            current_target: None,
            bubbles: true,
            cancelable: true,
            default_prevented: false,
            propagation_stopped: false,
        }
    }

    pub fn prevent_default(&mut self) {
        if self.cancelable {
            self.default_prevented = true;
        }
    }

    pub fn stop_propagation(&mut self) {
        self.propagation_stopped = true;
    }

    pub fn is_default_prevented(&self) -> bool {
        self.default_prevented
    }

    pub fn is_propagation_stopped(&self) -> bool {
        self.propagation_stopped
    }
}

pub struct EventListener {
    pub callback: Box<dyn Fn(&Event)>,
    pub once: bool,
}

impl EventListener {
    pub fn new(callback: Box<dyn Fn(&Event)>, once: bool) -> Self {
        EventListener { callback, once }
    }
}

pub struct EventTargetManager {
    pub listeners: HashMap<String, Vec<EventListener>>,
}

impl EventTargetManager {
    pub fn new() -> Self {
        EventTargetManager {
            listeners: HashMap::new(),
        }
    }

    pub fn add_event_listener(&mut self, type_: &str, listener: EventListener) {
        self.listeners
            .entry(type_.to_string())
            .or_insert_with(Vec::new)
            .push(listener);
    }

    pub fn remove_event_listener(&mut self, type_: &str, callback: &Box<dyn Fn(&Event)>) {
        if let Some(listeners) = self.listeners.get_mut(type_) {
            listeners.retain(|l| !std::ptr::eq(&l.callback, callback));
        }
    }

    pub fn dispatch_event(&mut self, event: &mut Event) {
        if let Some(listeners) = self.listeners.get_mut(&event.type_) {
            let mut to_remove = Vec::new();
            for (i, listener) in listeners.iter_mut().enumerate() {
                (listener.callback)(event);
                if listener.once {
                    to_remove.push(i);
                }
                if event.is_propagation_stopped() {
                    break;
                }
            }
            for &i in to_remove.iter().rev() {
                listeners.remove(i);
            }
        }
    }
}
