use crate::dom::node::Node;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct SelectionRange {
    pub node: Rc<RefCell<Node>>,
    pub start_offset: usize,
    pub end_offset: usize,
}

pub struct SelectionTracker {
    pub ranges: Vec<SelectionRange>,
    pub anchor_node: Option<Rc<RefCell<Node>>>,
    pub anchor_offset: usize,
    pub focus_node: Option<Rc<RefCell<Node>>>,
    pub focus_offset: usize,
    pub is_collapsed: bool,
    pub on_change: Option<Box<dyn Fn()>>,
}

impl SelectionTracker {
    pub fn new() -> Self {
        SelectionTracker {
            ranges: Vec::new(),
            anchor_node: None,
            anchor_offset: 0,
            focus_node: None,
            focus_offset: 0,
            is_collapsed: true,
            on_change: None,
        }
    }

    pub fn set_range(
        &mut self,
        node: Rc<RefCell<Node>>,
        start: usize,
        end: usize,
    ) {
        self.ranges.clear();
        self.ranges.push(SelectionRange {
            node: Rc::clone(&node),
            start_offset: start,
            end_offset: end,
        });
        self.anchor_node = Some(Rc::clone(&node));
        self.anchor_offset = start;
        self.focus_node = Some(node);
        self.focus_offset = end;
        self.is_collapsed = start == end;
        self.notify_change();
    }

    pub fn add_range(
        &mut self,
        node: Rc<RefCell<Node>>,
        start: usize,
        end: usize,
    ) {
        self.ranges.push(SelectionRange {
            node,
            start_offset: start,
            end_offset: end,
        });
        self.is_collapsed = false;
        self.notify_change();
    }

    pub fn select_all(&mut self, node: Rc<RefCell<Node>>, total_len: usize) {
        self.ranges.clear();
        self.ranges.push(SelectionRange {
            node: Rc::clone(&node),
            start_offset: 0,
            end_offset: total_len,
        });
        self.anchor_node = Some(Rc::clone(&node));
        self.anchor_offset = 0;
        self.focus_node = Some(node);
        self.focus_offset = total_len;
        self.is_collapsed = false;
        self.notify_change();
    }

    pub fn collapse_to_start(&mut self) {
        if let Some(range) = self.ranges.first() {
            self.focus_node = Some(range.node.clone());
            self.focus_offset = range.start_offset;
            self.is_collapsed = true;
        }
    }

    pub fn collapse_to_end(&mut self) {
        if let Some(range) = self.ranges.last() {
            self.focus_node = Some(range.node.clone());
            self.focus_offset = range.end_offset;
            self.is_collapsed = true;
        }
    }

    pub fn clear(&mut self) {
        self.ranges.clear();
        self.anchor_node = None;
        self.anchor_offset = 0;
        self.focus_node = None;
        self.focus_offset = 0;
        self.is_collapsed = true;
        self.notify_change();
    }

    pub fn range_count(&self) -> usize {
        self.ranges.len()
    }

    pub fn get_range(&self, index: usize) -> Option<&SelectionRange> {
        self.ranges.get(index)
    }

    pub fn selected_text(&self) -> String {
        self.ranges
            .first()
            .map(|r| {
                let borrowed = r.node.borrow();
                borrowed
                    .node_value
                    .clone()
                    .unwrap_or_default()
                    .chars()
                    .skip(r.start_offset)
                    .take(r.end_offset - r.start_offset)
                    .collect()
            })
            .unwrap_or_default()
    }

    pub fn selected_length(&self) -> usize {
        self.ranges
            .iter()
            .map(|r| r.end_offset.saturating_sub(r.start_offset))
            .sum()
    }

    pub fn set_on_change(&mut self, callback: Box<dyn Fn()>) {
        self.on_change = Some(callback);
    }

    fn notify_change(&self) {
        if let Some(ref cb) = self.on_change {
            cb();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_text(text: &str) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node::create_text(text)))
    }

    #[test]
    fn test_new() {
        let tracker = SelectionTracker::new();
        assert_eq!(tracker.range_count(), 0);
        assert!(tracker.is_collapsed);
    }

    #[test]
    fn test_set_range() {
        let mut tracker = SelectionTracker::new();
        let node = make_text("hello world");
        tracker.set_range(Rc::clone(&node), 0, 5);
        assert_eq!(tracker.range_count(), 1);
        assert_eq!(tracker.anchor_offset, 0);
        assert_eq!(tracker.focus_offset, 5);
        assert!(!tracker.is_collapsed);
    }

    #[test]
    fn test_set_range_collapsed() {
        let mut tracker = SelectionTracker::new();
        let node = make_text("hello");
        tracker.set_range(Rc::clone(&node), 3, 3);
        assert!(tracker.is_collapsed);
    }

    #[test]
    fn test_add_range() {
        let mut tracker = SelectionTracker::new();
        let node = make_text("hello");
        tracker.add_range(Rc::clone(&node), 0, 3);
        tracker.add_range(Rc::clone(&node), 5, 7);
        assert_eq!(tracker.range_count(), 2);
        assert!(!tracker.is_collapsed);
    }

    #[test]
    fn test_select_all() {
        let mut tracker = SelectionTracker::new();
        let node = make_text("hello");
        tracker.select_all(node, 5);
        assert_eq!(tracker.range_count(), 1);
        let range = tracker.get_range(0).unwrap();
        assert_eq!(range.start_offset, 0);
        assert_eq!(range.end_offset, 5);
        assert!(!tracker.is_collapsed);
    }

    #[test]
    fn test_collapse_to_start() {
        let mut tracker = SelectionTracker::new();
        let node = make_text("hello");
        tracker.set_range(Rc::clone(&node), 1, 4);
        tracker.collapse_to_start();
        assert!(tracker.is_collapsed);
        assert_eq!(tracker.focus_offset, 1);
    }

    #[test]
    fn test_collapse_to_end() {
        let mut tracker = SelectionTracker::new();
        let node = make_text("hello");
        tracker.set_range(Rc::clone(&node), 1, 4);
        tracker.collapse_to_end();
        assert!(tracker.is_collapsed);
        assert_eq!(tracker.focus_offset, 4);
    }

    #[test]
    fn test_clear() {
        let mut tracker = SelectionTracker::new();
        let node = make_text("hello");
        tracker.set_range(Rc::clone(&node), 0, 3);
        tracker.clear();
        assert_eq!(tracker.range_count(), 0);
        assert!(tracker.is_collapsed);
    }

    #[test]
    fn test_selected_text() {
        let mut tracker = SelectionTracker::new();
        let node = make_text("hello world");
        tracker.set_range(node, 6, 11);
        assert_eq!(tracker.selected_text(), "world");
    }

    #[test]
    fn test_selected_length() {
        let mut tracker = SelectionTracker::new();
        let node = make_text("hello");
        tracker.set_range(Rc::clone(&node), 1, 4);
        assert_eq!(tracker.selected_length(), 3);
    }

    #[test]
    fn test_get_range_out_of_bounds() {
        let tracker = SelectionTracker::new();
        assert!(tracker.get_range(0).is_none());
    }

    #[test]
    fn test_on_change_callback() {
        use std::cell::Cell;
        use std::rc::Rc as StdRc;
        let mut tracker = SelectionTracker::new();
        let counter = StdRc::new(Cell::new(0u32));
        let c = StdRc::clone(&counter);
        tracker.set_on_change(Box::new(move || { c.set(c.get() + 1); }));
        let node = make_text("hi");
        tracker.set_range(node, 0, 2);
        assert_eq!(counter.get(), 1);
        tracker.clear();
        assert_eq!(counter.get(), 2);
    }
}
