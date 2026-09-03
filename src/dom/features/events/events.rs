use crate::dom::event::{Event, EventListener, EventTargetManager};
use std::collections::HashMap;

pub struct EventDispatcher {
    pub node_managers: HashMap<usize, EventTargetManager>,
    pub id_counter: usize,
}

impl EventDispatcher {
    pub fn new() -> Self {
        EventDispatcher {
            node_managers: HashMap::new(),
            id_counter: 0,
        }
    }

    pub fn register_node(&mut self) -> usize {
        let id = self.id_counter;
        self.node_managers.insert(id, EventTargetManager::new());
        self.id_counter += 1;
        id
    }

    pub fn add_event_listener(
        &mut self,
        node_id: usize,
        event_type: &str,
        callback: Box<dyn Fn(&Event)>,
        once: bool,
    ) {
        if let Some(mgr) = self.node_managers.get_mut(&node_id) {
            let listener = EventListener::new(callback, once);
            mgr.add_event_listener(event_type, listener);
        }
    }

    pub fn remove_event_listener(&mut self, node_id: usize, event_type: &str, listener_ptr: usize) {
        if let Some(mgr) = self.node_managers.get_mut(&node_id) {
            if let Some(listeners) = mgr.listeners.get_mut(event_type) {
                listeners.retain(|l| {
                    let p: *const dyn Fn(&Event) = &*l.callback;
                    (p as *const ()) as usize != listener_ptr
                });
            }
        }
    }

    pub fn dispatch_event(&mut self, node_id: usize, event: &mut Event) {
        if let Some(mgr) = self.node_managers.get_mut(&node_id) {
            mgr.dispatch_event(event);
        }
    }

    pub fn dispatch_event_bubble_chain(&mut self, chain: &[usize], event: &mut Event) {
        for &id in chain {
            event.current_target = Some(crate::dom::event::EventTarget { node: Some(id) });
            self.dispatch_event(id, event);
            if event.propagation_stopped {
                break;
            }
        }
    }

    pub fn unregister_node(&mut self, node_id: usize) {
        self.node_managers.remove(&node_id);
    }

    pub fn has_listeners(&self, node_id: usize, event_type: &str) -> bool {
        self.node_managers.get(&node_id).map_or(false, |mgr| {
            mgr.listeners
                .get(event_type)
                .map_or(false, |v| !v.is_empty())
        })
    }

    pub fn listener_count(&self, node_id: usize, event_type: &str) -> usize {
        self.node_managers
            .get(&node_id)
            .and_then(|mgr| mgr.listeners.get(event_type))
            .map_or(0, |v| v.len())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell;
    use std::rc::Rc;

    #[test]
    fn test_new() {
        let _d = EventDispatcher::new();
    }

    #[test]
    fn test_register_node() {
        let mut d = EventDispatcher::new();
        let id = d.register_node();
        assert_eq!(id, 0);
        let id2 = d.register_node();
        assert_eq!(id2, 1);
    }

    #[test]
    fn test_add_and_dispatch() {
        let mut d = EventDispatcher::new();
        let id = d.register_node();
        let counter = Rc::new(Cell::new(0u32));
        let c = Rc::clone(&counter);
        d.add_event_listener(
            id,
            "click",
            Box::new(move |_e| {
                c.set(c.get() + 1);
            }),
            false,
        );

        let mut event = Event::new("click");
        d.dispatch_event(id, &mut event);
        assert_eq!(counter.get(), 1);

        d.dispatch_event(id, &mut event);
        assert_eq!(counter.get(), 2);
    }

    #[test]
    fn test_once_listener() {
        let mut d = EventDispatcher::new();
        let id = d.register_node();
        let counter = Rc::new(Cell::new(0u32));
        let c = Rc::clone(&counter);
        d.add_event_listener(
            id,
            "load",
            Box::new(move |_e| {
                c.set(c.get() + 1);
            }),
            true,
        );

        let mut event = Event::new("load");
        d.dispatch_event(id, &mut event);
        assert_eq!(counter.get(), 1);
        assert_eq!(d.listener_count(id, "load"), 0);

        let mut event2 = Event::new("load");
        d.dispatch_event(id, &mut event2);
        assert_eq!(counter.get(), 1);
    }

    #[test]
    fn test_stop_propagation() {
        let mut d = EventDispatcher::new();
        let id = d.register_node();
        let counter = Rc::new(Cell::new(0u32));
        let c = Rc::clone(&counter);
        d.add_event_listener(
            id,
            "click",
            Box::new(move |_e| {
                c.set(c.get() + 1);
            }),
            false,
        );

        let mut event = Event::new("click");
        event.propagation_stopped = true;
        d.dispatch_event(id, &mut event);
        assert_eq!(counter.get(), 1);
        assert!(event.propagation_stopped);
    }

    #[test]
    fn test_bubble_chain() {
        let mut d = EventDispatcher::new();
        let child_id = d.register_node();
        let parent_id = d.register_node();
        let counter = Rc::new(Cell::new(0u32));
        let c1 = Rc::clone(&counter);
        let c2 = Rc::clone(&counter);

        d.add_event_listener(
            child_id,
            "click",
            Box::new(move |_e| {
                c1.set(c1.get() + 1);
            }),
            false,
        );
        d.add_event_listener(
            parent_id,
            "click",
            Box::new(move |_e| {
                c2.set(c2.get() + 1);
            }),
            false,
        );

        let mut event = Event::new("click");
        d.dispatch_event_bubble_chain(&[child_id, parent_id], &mut event);
        assert_eq!(counter.get(), 2);
    }

    #[test]
    fn test_bubble_chain_stops() {
        let mut d = EventDispatcher::new();
        let child_id = d.register_node();
        let parent_id = d.register_node();
        let parent_hit = Rc::new(Cell::new(false));
        let pa = Rc::clone(&parent_hit);

        d.add_event_listener(child_id, "click", Box::new(move |_e| {}), false);
        d.add_event_listener(
            parent_id,
            "click",
            Box::new(move |_e| {
                pa.set(true);
            }),
            false,
        );

        let mut event = Event::new("click");
        event.propagation_stopped = true;
        d.dispatch_event_bubble_chain(&[child_id, parent_id], &mut event);
        assert!(event.propagation_stopped);
        assert!(!parent_hit.get());
    }

    #[test]
    fn test_unregister_node() {
        let mut d = EventDispatcher::new();
        let id = d.register_node();
        d.add_event_listener(id, "x", Box::new(|_| {}), false);
        assert!(d.has_listeners(id, "x"));
        d.unregister_node(id);
        assert!(!d.has_listeners(id, "x"));
    }

    #[test]
    fn test_has_listeners() {
        let mut d = EventDispatcher::new();
        let id = d.register_node();
        assert!(!d.has_listeners(id, "click"));
        d.add_event_listener(id, "click", Box::new(|_| {}), false);
        assert!(d.has_listeners(id, "click"));
        assert!(!d.has_listeners(id, "hover"));
    }

    #[test]
    fn test_listener_count() {
        let mut d = EventDispatcher::new();
        let id = d.register_node();
        assert_eq!(d.listener_count(id, "x"), 0);
        d.add_event_listener(id, "x", Box::new(|_| {}), false);
        d.add_event_listener(id, "x", Box::new(|_| {}), false);
        assert_eq!(d.listener_count(id, "x"), 2);
    }

    #[test]
    fn test_dispatch_no_listeners() {
        let mut d = EventDispatcher::new();
        let id = d.register_node();
        let mut event = Event::new("missing");
        d.dispatch_event(id, &mut event);
    }
}
