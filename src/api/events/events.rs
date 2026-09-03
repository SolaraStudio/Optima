use std::collections::HashMap;

pub type EventCallback = Box<dyn Fn(&str) + Send + Sync>;

pub struct EventDispatcher {
    listeners: HashMap<String, Vec<EventCallback>>,
}

impl EventDispatcher {
    pub fn new() -> Self {
        EventDispatcher {
            listeners: HashMap::new(),
        }
    }

    pub fn add_listener<F>(&mut self, event_type: &str, callback: F)
    where
        F: Fn(&str) + Send + Sync + 'static,
    {
        self.listeners
            .entry(event_type.to_string())
            .or_default()
            .push(Box::new(callback));
    }

    pub fn dispatch(&self, event_type: &str) {
        if let Some(callbacks) = self.listeners.get(event_type) {
            for cb in callbacks {
                cb(event_type);
            }
        }
    }

    pub fn clear(&mut self) {
        self.listeners.clear();
    }

    pub fn remove_listeners(&mut self, event_type: &str) {
        self.listeners.remove(event_type);
    }
}

impl Default for EventDispatcher {
    fn default() -> Self {
        Self::new()
    }
}
