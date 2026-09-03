use crate::dom::mutation::MutationRecord;
use crate::dom::node::Node;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub struct MutationCallbackEntry {
    pub callback: Box<dyn Fn(&[MutationRecord])>,
    pub child_list: bool,
    pub attributes: bool,
    pub subtree: bool,
    pub attribute_filter: Option<Vec<String>>,
}

pub struct MutationTracker {
    pub observers: HashMap<usize, MutationCallbackEntry>,
    pub pending_records: Vec<MutationRecord>,
    pub id_counter: usize,
}

impl Default for MutationTracker {
    fn default() -> Self {
        Self::new()
    }
}

impl MutationTracker {
    pub fn new() -> Self {
        MutationTracker {
            observers: HashMap::new(),
            pending_records: Vec::new(),
            id_counter: 0,
        }
    }

    pub fn observe(
        &mut self,
        callback: Box<dyn Fn(&[MutationRecord])>,
        child_list: bool,
        attributes: bool,
        subtree: bool,
    ) -> usize {
        let id = self.id_counter;
        self.observers.insert(
            id,
            MutationCallbackEntry {
                callback,
                child_list,
                attributes,
                subtree,
                attribute_filter: None,
            },
        );
        self.id_counter += 1;
        id
    }

    pub fn observe_with_filter(
        &mut self,
        callback: Box<dyn Fn(&[MutationRecord])>,
        child_list: bool,
        attributes: bool,
        subtree: bool,
        attribute_filter: Vec<String>,
    ) -> usize {
        let id = self.id_counter;
        self.observers.insert(
            id,
            MutationCallbackEntry {
                callback,
                child_list,
                attributes,
                subtree,
                attribute_filter: Some(attribute_filter),
            },
        );
        self.id_counter += 1;
        id
    }

    pub fn disconnect(&mut self, observer_id: usize) {
        self.observers.remove(&observer_id);
    }

    pub fn disconnect_all(&mut self) {
        self.observers.clear();
        self.pending_records.clear();
    }

    pub fn notify_child_added(&mut self, parent: &Rc<RefCell<Node>>, child: &Rc<RefCell<Node>>) {
        let mut record = MutationRecord::new("childList", Rc::clone(parent));
        record.added_nodes = vec![Rc::clone(child)];
        self.enqueue_record(record);
    }

    pub fn notify_child_removed(&mut self, parent: &Rc<RefCell<Node>>, child: &Rc<RefCell<Node>>) {
        let mut record = MutationRecord::new("childList", Rc::clone(parent));
        record.removed_nodes = vec![Rc::clone(child)];
        self.enqueue_record(record);
    }

    pub fn notify_attribute_changed(
        &mut self,
        target: &Rc<RefCell<Node>>,
        attr_name: &str,
        old_value: &str,
    ) {
        let mut record = MutationRecord::new("attributes", Rc::clone(target));
        record = record.with_attribute(attr_name, old_value);
        self.enqueue_record(record);
    }

    pub fn flush(&mut self) {
        let records: Vec<MutationRecord> = std::mem::take(&mut self.pending_records);
        if records.is_empty() {
            return;
        }
        for entry in self.observers.values() {
            let relevant: Vec<MutationRecord> = records
                .iter()
                .filter(|r| self.record_matches(r, entry))
                .cloned()
                .collect();
            if !relevant.is_empty() {
                (entry.callback)(&relevant);
            }
        }
    }

    fn record_matches(&self, record: &MutationRecord, entry: &MutationCallbackEntry) -> bool {
        match record.type_.as_str() {
            "childList" => entry.child_list,
            "attributes" => {
                if !entry.attributes {
                    return false;
                }
                if let Some(ref filter) = entry.attribute_filter {
                    if let Some(ref attr_name) = record.attribute_name {
                        return filter.contains(attr_name);
                    }
                    return false;
                }
                true
            }
            _ => false,
        }
    }

    fn enqueue_record(&mut self, record: MutationRecord) {
        self.pending_records.push(record);
    }

    pub fn pending_count(&self) -> usize {
        self.pending_records.len()
    }

    pub fn observer_count(&self) -> usize {
        self.observers.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell;
    use std::rc::Rc;

    fn make_node() -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node::create_element("div")))
    }

    #[test]
    fn test_new() {
        let tracker = MutationTracker::new();
        assert_eq!(tracker.observer_count(), 0);
        assert_eq!(tracker.pending_count(), 0);
    }

    #[test]
    fn test_observe() {
        let mut tracker = MutationTracker::new();
        let id = tracker.observe(Box::new(|_| {}), true, false, false);
        assert_eq!(id, 0);
        assert_eq!(tracker.observer_count(), 1);
    }

    #[test]
    fn test_observe_with_filter() {
        let mut tracker = MutationTracker::new();
        let id = tracker.observe_with_filter(
            Box::new(|_| {}),
            false,
            true,
            false,
            vec!["class".to_string()],
        );
        assert_eq!(id, 0);
        assert_eq!(tracker.observer_count(), 1);
    }

    #[test]
    fn test_notify_child_added() {
        let mut tracker = MutationTracker::new();
        let counter = Rc::new(Cell::new(0u32));
        let c = Rc::clone(&counter);
        tracker.observe(
            Box::new(move |records| {
                c.set(c.get() + records.len() as u32);
            }),
            true,
            false,
            false,
        );

        let parent = make_node();
        let child = make_node();
        tracker.notify_child_added(&parent, &child);
        assert_eq!(tracker.pending_count(), 1);
        tracker.flush();
        assert_eq!(counter.get(), 1);
        assert_eq!(tracker.pending_count(), 0);
    }

    #[test]
    fn test_notify_child_removed() {
        let mut tracker = MutationTracker::new();
        let counter = Rc::new(Cell::new(0u32));
        let c = Rc::clone(&counter);
        tracker.observe(
            Box::new(move |records| {
                c.set(c.get() + records.len() as u32);
            }),
            true,
            false,
            false,
        );

        let parent = make_node();
        let child = make_node();
        tracker.notify_child_removed(&parent, &child);
        tracker.flush();
        assert_eq!(counter.get(), 1);
    }

    #[test]
    fn test_notify_attribute_changed() {
        let mut tracker = MutationTracker::new();
        let counter = Rc::new(Cell::new(0u32));
        let c = Rc::clone(&counter);
        tracker.observe(
            Box::new(move |records| {
                c.set(c.get() + records.len() as u32);
            }),
            false,
            true,
            false,
        );

        let target = make_node();
        tracker.notify_attribute_changed(&target, "class", "old");
        tracker.flush();
        assert_eq!(counter.get(), 1);
    }

    #[test]
    fn test_attribute_filter_matches() {
        let mut tracker = MutationTracker::new();
        let counter = Rc::new(Cell::new(0u32));
        let c = Rc::clone(&counter);
        tracker.observe_with_filter(
            Box::new(move |records| {
                c.set(c.get() + records.len() as u32);
            }),
            false,
            true,
            false,
            vec!["class".to_string()],
        );

        let target = make_node();
        tracker.notify_attribute_changed(&target, "class", "old");
        tracker.flush();
        assert_eq!(counter.get(), 1);
    }

    #[test]
    fn test_attribute_filter_no_match() {
        let mut tracker = MutationTracker::new();
        let counter = Rc::new(Cell::new(0u32));
        let c = Rc::clone(&counter);
        tracker.observe_with_filter(
            Box::new(move |records| {
                c.set(c.get() + records.len() as u32);
            }),
            false,
            true,
            false,
            vec!["id".to_string()],
        );

        let target = make_node();
        tracker.notify_attribute_changed(&target, "class", "old");
        tracker.flush();
        assert_eq!(counter.get(), 0);
    }

    #[test]
    fn test_child_list_observer_ignores_attribute() {
        let mut tracker = MutationTracker::new();
        let counter = Rc::new(Cell::new(0u32));
        let c = Rc::clone(&counter);
        tracker.observe(
            Box::new(move |records| {
                c.set(c.get() + records.len() as u32);
            }),
            true,
            false,
            false,
        );

        let target = make_node();
        tracker.notify_attribute_changed(&target, "class", "old");
        tracker.flush();
        assert_eq!(counter.get(), 0);
    }

    #[test]
    fn test_disconnect() {
        let mut tracker = MutationTracker::new();
        let id = tracker.observe(Box::new(|_| {}), true, false, false);
        tracker.disconnect(id);
        assert_eq!(tracker.observer_count(), 0);
    }

    #[test]
    fn test_disconnect_all() {
        let mut tracker = MutationTracker::new();
        tracker.observe(Box::new(|_| {}), true, false, false);
        tracker.observe(Box::new(|_| {}), true, false, false);
        let parent = make_node();
        let child = make_node();
        tracker.notify_child_added(&parent, &child);
        tracker.disconnect_all();
        assert_eq!(tracker.observer_count(), 0);
        assert_eq!(tracker.pending_count(), 0);
    }

    #[test]
    fn test_flush_empty() {
        let mut tracker = MutationTracker::new();
        let counter = Rc::new(Cell::new(0u32));
        let c = Rc::clone(&counter);
        tracker.observe(
            Box::new(move |_| {
                c.set(c.get() + 1);
            }),
            true,
            false,
            false,
        );
        tracker.flush();
        assert_eq!(counter.get(), 0);
    }

    #[test]
    fn test_multiple_observers() {
        let mut tracker = MutationTracker::new();
        let counter = Rc::new(Cell::new(0u32));
        let c1 = Rc::clone(&counter);
        let c2 = Rc::clone(&counter);
        tracker.observe(
            Box::new(move |r| {
                c1.set(c1.get() + r.len() as u32);
            }),
            true,
            false,
            false,
        );
        tracker.observe(
            Box::new(move |r| {
                c2.set(c2.get() + r.len() as u32);
            }),
            true,
            false,
            false,
        );

        let parent = make_node();
        let child = make_node();
        tracker.notify_child_added(&parent, &child);
        tracker.flush();
        assert_eq!(counter.get(), 2);
    }
}
