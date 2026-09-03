use crate::dom::range::Range;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Selection {
    pub ranges: Vec<Range>,
    pub anchor_node: Option<Rc<RefCell<crate::dom::node::Node>>>,
    pub anchor_offset: u32,
    pub focus_node: Option<Rc<RefCell<crate::dom::node::Node>>>,
    pub focus_offset: u32,
}

impl Selection {
    pub fn new() -> Self {
        Selection {
            ranges: Vec::new(),
            anchor_node: None,
            anchor_offset: 0,
            focus_node: None,
            focus_offset: 0,
        }
    }

    pub fn add_range(&mut self, range: Range) {
        self.ranges.push(range);
        if self.ranges.len() > 1 {
            self.ranges.remove(0);
        }
        if let Some(range) = self.ranges.last() {
            self.anchor_node = range.start_container.clone();
            self.anchor_offset = range.start_offset;
            self.focus_node = range.end_container.clone();
            self.focus_offset = range.end_offset;
        }
    }

    pub fn remove_all_ranges(&mut self) {
        self.ranges.clear();
        self.anchor_node = None;
        self.anchor_offset = 0;
        self.focus_node = None;
        self.focus_offset = 0;
    }

    pub fn get_range_at(&self, index: usize) -> Option<&Range> {
        self.ranges.get(index)
    }

    pub fn range_count(&self) -> usize {
        self.ranges.len()
    }

    pub fn is_collapsed(&self) -> bool {
        if let Some(range) = self.ranges.last() {
            range.collapsed
        } else {
            true
        }
    }

    pub fn to_string(&self) -> String {
        if let Some(range) = self.ranges.last() {
            range.to_string()
        } else {
            String::new()
        }
    }
}
