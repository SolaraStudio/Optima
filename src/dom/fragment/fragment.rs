use crate::dom::node::{Node, NodeType};
use std::cell::RefCell;
use std::rc::Rc;

pub struct DocumentFragment {
    pub node: Rc<RefCell<Node>>,
}

impl DocumentFragment {
    pub fn new() -> Self {
        let node = Rc::new(RefCell::new(Node::new(NodeType::DocumentFragment)));
        DocumentFragment { node }
    }

    pub fn append_child(&self, child: Rc<RefCell<Node>>) -> Rc<RefCell<Node>> {
        self.node.borrow_mut().append_child(child)
    }

    pub fn remove_child(&self, child: &Rc<RefCell<Node>>) -> Option<Rc<RefCell<Node>>> {
        self.node.borrow_mut().remove_child(child)
    }

    pub fn child_nodes(&self) -> Vec<Rc<RefCell<Node>>> {
        self.node.borrow().children.clone()
    }
}
