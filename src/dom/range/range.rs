use crate::dom::node::Node;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Range {
    pub start_container: Option<Rc<RefCell<Node>>>,
    pub start_offset: u32,
    pub end_container: Option<Rc<RefCell<Node>>>,
    pub end_offset: u32,
    pub collapsed: bool,
}

impl Range {
    pub fn new() -> Self {
        Range {
            start_container: None,
            start_offset: 0,
            end_container: None,
            end_offset: 0,
            collapsed: true,
        }
    }

    pub fn set_start(&mut self, container: Rc<RefCell<Node>>, offset: u32) {
        self.start_container = Some(container);
        self.start_offset = offset;
        self.update_collapsed();
    }

    pub fn set_end(&mut self, container: Rc<RefCell<Node>>, offset: u32) {
        self.end_container = Some(container);
        self.end_offset = offset;
        self.update_collapsed();
    }

    pub fn set_start_before(&mut self, node: Rc<RefCell<Node>>) {
        if let Some(parent) = node.borrow().parent.clone() {
            let index = parent
                .borrow()
                .children
                .iter()
                .position(|c| Rc::ptr_eq(c, &node));
            if let Some(idx) = index {
                self.set_start(parent, idx as u32);
            }
        }
    }

    pub fn set_start_after(&mut self, node: Rc<RefCell<Node>>) {
        if let Some(parent) = node.borrow().parent.clone() {
            let index = parent
                .borrow()
                .children
                .iter()
                .position(|c| Rc::ptr_eq(c, &node));
            if let Some(idx) = index {
                self.set_start(parent, (idx + 1) as u32);
            }
        }
    }

    pub fn set_end_before(&mut self, node: Rc<RefCell<Node>>) {
        if let Some(parent) = node.borrow().parent.clone() {
            let index = parent
                .borrow()
                .children
                .iter()
                .position(|c| Rc::ptr_eq(c, &node));
            if let Some(idx) = index {
                self.set_end(parent, idx as u32);
            }
        }
    }

    pub fn set_end_after(&mut self, node: Rc<RefCell<Node>>) {
        if let Some(parent) = node.borrow().parent.clone() {
            let index = parent
                .borrow()
                .children
                .iter()
                .position(|c| Rc::ptr_eq(c, &node));
            if let Some(idx) = index {
                self.set_end(parent, (idx + 1) as u32);
            }
        }
    }

    pub fn select_node(&mut self, node: Rc<RefCell<Node>>) {
        if let Some(parent) = node.borrow().parent.clone() {
            let index = parent
                .borrow()
                .children
                .iter()
                .position(|c| Rc::ptr_eq(c, &node));
            if let Some(idx) = index {
                self.set_start(parent.clone(), idx as u32);
                self.set_end(parent, (idx + 1) as u32);
            }
        }
    }

    pub fn select_node_contents(&mut self, node: Rc<RefCell<Node>>) {
        self.set_start(node.clone(), 0);
        self.set_end(node.clone(), node.borrow().children.len() as u32);
    }

    pub fn collapse(&mut self, to_start: bool) {
        if to_start {
            self.end_container = self.start_container.clone();
            self.end_offset = self.start_offset;
        } else {
            self.start_container = self.end_container.clone();
            self.start_offset = self.end_offset;
        }
        self.collapsed = true;
    }

    pub fn clone_range(&self) -> Self {
        Range {
            start_container: self.start_container.clone(),
            start_offset: self.start_offset,
            end_container: self.end_container.clone(),
            end_offset: self.end_offset,
            collapsed: self.collapsed,
        }
    }

    pub fn to_string(&self) -> String {
        let text = String::new();
        // Simplified: traverse nodes between start and end
        // Full implementation would be more complex
        text
    }

    fn update_collapsed(&mut self) {
        if let (Some(start_container), Some(end_container)) =
            (&self.start_container, &self.end_container)
        {
            if Rc::ptr_eq(start_container, end_container) && self.start_offset == self.end_offset {
                self.collapsed = true;
            } else {
                self.collapsed = false;
            }
        } else {
            self.collapsed = true;
        }
    }
}
