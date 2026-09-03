use std::collections::HashMap;

pub enum LifecycleEvent {
    DomContentLoaded,
    Load,
    BeforeUnload,
}

pub struct LifecycleEntry {
    pub phase: LifecyclePhase,
    pub callback: Box<dyn FnMut(&LifecycleEvent)>,
}

pub enum LifecyclePhase {
    BeforeUnload,
    DomContentLoaded,
    Load,
}

pub struct LifecycleManager {
    pub listeners: HashMap<String, Vec<LifecycleEntry>>,
    pub fired: HashMap<String, bool>,
    pub document_ready: bool,
    pub window_loaded: bool,
}

impl LifecycleManager {
    pub fn new() -> Self {
        LifecycleManager {
            listeners: HashMap::new(),
            fired: HashMap::new(),
            document_ready: false,
            window_loaded: false,
        }
    }

    pub fn on_dom_content_loaded(&mut self, callback: Box<dyn FnMut(&LifecycleEvent)>) {
        let entry = LifecycleEntry {
            phase: LifecyclePhase::DomContentLoaded,
            callback,
        };
        self.listeners
            .entry("DOMContentLoaded".to_string())
            .or_insert_with(Vec::new)
            .push(entry);

        if self.document_ready {
            self.fire_pending("DOMContentLoaded", &LifecycleEvent::DomContentLoaded);
        }
    }

    pub fn on_load(&mut self, callback: Box<dyn FnMut(&LifecycleEvent)>) {
        let entry = LifecycleEntry {
            phase: LifecyclePhase::Load,
            callback,
        };
        self.listeners
            .entry("load".to_string())
            .or_insert_with(Vec::new)
            .push(entry);

        if self.window_loaded {
            self.fire_pending("load", &LifecycleEvent::Load);
        }
    }

    pub fn on_beforeunload(&mut self, callback: Box<dyn FnMut(&LifecycleEvent)>) {
        let entry = LifecycleEntry {
            phase: LifecyclePhase::BeforeUnload,
            callback,
        };
        self.listeners
            .entry("beforeunload".to_string())
            .or_insert_with(Vec::new)
            .push(entry);
    }

    pub fn emit_dom_content_loaded(&mut self) {
        self.document_ready = true;
        let event = LifecycleEvent::DomContentLoaded;
        self.fire_all("DOMContentLoaded", &event);
        self.fired.insert("DOMContentLoaded".to_string(), true);
    }

    pub fn emit_load(&mut self) {
        self.window_loaded = true;
        let event = LifecycleEvent::Load;
        self.fire_all("load", &event);
        self.fired.insert("load".to_string(), true);
    }

    pub fn emit_beforeunload(&mut self) -> bool {
        let event = LifecycleEvent::BeforeUnload;
        self.fire_all("beforeunload", &event);
        true
    }

    fn fire_all(&mut self, key: &str, event: &LifecycleEvent) {
        if let Some(listeners) = self.listeners.get_mut(key) {
            for entry in listeners.iter_mut() {
                (entry.callback)(event);
            }
        }
    }

    fn fire_pending(&mut self, key: &str, event: &LifecycleEvent) {
        self.fire_all(key, event);
    }

    pub fn has_listeners(&self, key: &str) -> bool {
        self.listeners.get(key).map_or(false, |v| !v.is_empty())
    }

    pub fn listener_count(&self, key: &str) -> usize {
        self.listeners.get(key).map_or(0, |v| v.len())
    }

    pub fn clear_listeners(&mut self, key: &str) {
        self.listeners.remove(key);
    }

    pub fn is_document_ready(&self) -> bool {
        self.document_ready
    }

    pub fn is_window_loaded(&self) -> bool {
        self.window_loaded
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell;
    use std::rc::Rc;

    #[test]
    fn test_new() {
        let mgr = LifecycleManager::new();
        assert!(!mgr.is_document_ready());
        assert!(!mgr.is_window_loaded());
    }

    #[test]
    fn test_emit_dom_content_loaded() {
        let mut mgr = LifecycleManager::new();
        let counter = Rc::new(Cell::new(0u32));
        let c = Rc::clone(&counter);
        mgr.on_dom_content_loaded(Box::new(move |_e| {
            c.set(c.get() + 1);
        }));
        mgr.emit_dom_content_loaded();
        assert_eq!(counter.get(), 1);
        assert!(mgr.is_document_ready());
    }

    #[test]
    fn test_emit_load() {
        let mut mgr = LifecycleManager::new();
        let counter = Rc::new(Cell::new(0u32));
        let c = Rc::clone(&counter);
        mgr.on_load(Box::new(move |_e| {
            c.set(c.get() + 1);
        }));
        mgr.emit_load();
        assert_eq!(counter.get(), 1);
        assert!(mgr.is_window_loaded());
    }

    #[test]
    fn test_emit_beforeunload() {
        let mut mgr = LifecycleManager::new();
        let counter = Rc::new(Cell::new(0u32));
        let c = Rc::clone(&counter);
        mgr.on_beforeunload(Box::new(move |_e| {
            c.set(c.get() + 1);
        }));
        let result = mgr.emit_beforeunload();
        assert!(result);
        assert_eq!(counter.get(), 1);
    }

    #[test]
    fn test_listener_after_emit() {
        let mut mgr = LifecycleManager::new();
        mgr.emit_dom_content_loaded();
        let counter = Rc::new(Cell::new(0u32));
        let c = Rc::clone(&counter);
        mgr.on_dom_content_loaded(Box::new(move |_e| {
            c.set(c.get() + 1);
        }));
        assert_eq!(counter.get(), 1);
    }

    #[test]
    fn test_multiple_listeners() {
        let mut mgr = LifecycleManager::new();
        let counter = Rc::new(Cell::new(0u32));
        let c1 = Rc::clone(&counter);
        let c2 = Rc::clone(&counter);
        mgr.on_load(Box::new(move |_e| {
            c1.set(c1.get() + 1);
        }));
        mgr.on_load(Box::new(move |_e| {
            c2.set(c2.get() + 1);
        }));
        mgr.emit_load();
        assert_eq!(counter.get(), 2);
    }

    #[test]
    fn test_has_listeners() {
        let mut mgr = LifecycleManager::new();
        assert!(!mgr.has_listeners("DOMContentLoaded"));
        mgr.on_dom_content_loaded(Box::new(|_| {}));
        assert!(mgr.has_listeners("DOMContentLoaded"));
    }

    #[test]
    fn test_listener_count() {
        let mut mgr = LifecycleManager::new();
        assert_eq!(mgr.listener_count("load"), 0);
        mgr.on_load(Box::new(|_| {}));
        mgr.on_load(Box::new(|_| {}));
        assert_eq!(mgr.listener_count("load"), 2);
    }

    #[test]
    fn test_clear_listeners() {
        let mut mgr = LifecycleManager::new();
        mgr.on_load(Box::new(|_| {}));
        mgr.on_load(Box::new(|_| {}));
        mgr.clear_listeners("load");
        assert!(!mgr.has_listeners("load"));
    }

    #[test]
    fn test_emit_load_before_listener() {
        let mut mgr = LifecycleManager::new();
        mgr.emit_load();
        let counter = Rc::new(Cell::new(0u32));
        let c = Rc::clone(&counter);
        mgr.on_load(Box::new(move |_e| {
            c.set(c.get() + 1);
        }));
        assert_eq!(counter.get(), 1);
    }
}
