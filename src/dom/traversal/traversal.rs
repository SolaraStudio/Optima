use crate::dom::node::Node;
use std::cell::RefCell;
use std::rc::Rc;

pub struct TreeWalker {
    pub root: Rc<RefCell<Node>>,
    pub current_node: Rc<RefCell<Node>>,
    pub filter: Option<Box<dyn Fn(&Rc<RefCell<Node>>) -> bool>>,
}

impl TreeWalker {
    pub fn new(root: Rc<RefCell<Node>>) -> Self {
        TreeWalker {
            root: Rc::clone(&root),
            current_node: Rc::clone(&root),
            filter: None,
        }
    }

    pub fn with_filter(mut self, filter: Box<dyn Fn(&Rc<RefCell<Node>>) -> bool>) -> Self {
        self.filter = Some(filter);
        self
    }

    pub fn set_current_node(&mut self, node: Rc<RefCell<Node>>) {
        self.current_node = node;
    }

    pub fn get_current_node(&self) -> Rc<RefCell<Node>> {
        Rc::clone(&self.current_node)
    }

    pub fn parent_node(&mut self) -> Option<Rc<RefCell<Node>>> {
        let parent = self.current_node.borrow().parent.clone();
        if let Some(parent) = parent {
            self.current_node = Rc::clone(&parent);
            Some(parent)
        } else {
            None
        }
    }

    pub fn first_child(&mut self) -> Option<Rc<RefCell<Node>>> {
        let children = self.current_node.borrow().children.clone();
        if let Some(first) = children.first() {
            self.current_node = Rc::clone(first);
            Some(Rc::clone(first))
        } else {
            None
        }
    }

    pub fn last_child(&mut self) -> Option<Rc<RefCell<Node>>> {
        let children = self.current_node.borrow().children.clone();
        if let Some(last) = children.last() {
            self.current_node = Rc::clone(last);
            Some(Rc::clone(last))
        } else {
            None
        }
    }

    pub fn next_sibling(&mut self) -> Option<Rc<RefCell<Node>>> {
        let parent = self.current_node.borrow().parent.clone();
        if let Some(parent) = parent {
            let children = parent.borrow().children.clone();
            let current = Rc::clone(&self.current_node);
            for (i, child) in children.iter().enumerate() {
                if Rc::ptr_eq(child, &current) {
                    if i + 1 < children.len() {
                        self.current_node = Rc::clone(&children[i + 1]);
                        return Some(Rc::clone(&children[i + 1]));
                    }
                    break;
                }
            }
        }
        None
    }

    pub fn previous_sibling(&mut self) -> Option<Rc<RefCell<Node>>> {
        let parent = self.current_node.borrow().parent.clone();
        if let Some(parent) = parent {
            let children = parent.borrow().children.clone();
            let current = Rc::clone(&self.current_node);
            for (i, child) in children.iter().enumerate() {
                if Rc::ptr_eq(child, &current) {
                    if i > 0 {
                        self.current_node = Rc::clone(&children[i - 1]);
                        return Some(Rc::clone(&children[i - 1]));
                    }
                    break;
                }
            }
        }
        None
    }

    pub fn next_node(&mut self) -> Option<Rc<RefCell<Node>>> {
        if let Some(first_child) = self.first_child() {
            return Some(first_child);
        }
        if let Some(next_sibling) = self.next_sibling() {
            return Some(next_sibling);
        }
        let mut current = Rc::clone(&self.current_node);
        let mut next_parent = current.borrow().parent.clone();
        while let Some(parent) = next_parent {
            if let Some(next_sib) = self.next_sibling() {
                return Some(next_sib);
            }
            current = parent;
            next_parent = current.borrow().parent.clone();
        }
        None
    }

    pub fn previous_node(&mut self) -> Option<Rc<RefCell<Node>>> {
        if let Some(prev_sibling) = self.previous_sibling() {
            let mut node = prev_sibling;
            loop {
                let last = node.borrow().children.last().cloned();
                match last {
                    Some(last_child) => node = last_child,
                    None => break,
                }
            }
            self.current_node = node;
            return Some(Rc::clone(&self.current_node));
        }
        if let Some(parent) = self.parent_node() {
            return Some(parent);
        }
        None
    }

    pub fn accept_node(&self, node: &Rc<RefCell<Node>>) -> bool {
        if let Some(filter) = &self.filter {
            filter(node)
        } else {
            true
        }
    }
}
