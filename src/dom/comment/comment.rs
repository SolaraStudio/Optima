use crate::dom::node::Node;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Comment {
    pub node: Rc<RefCell<Node>>,
}

impl Comment {
    pub fn new(text: &str) -> Self {
        let node = Rc::new(RefCell::new(Node::create_comment(text)));
        Comment { node }
    }

    pub fn from_node(node: Rc<RefCell<Node>>) -> Self {
        Comment { node }
    }

    pub fn get_data(&self) -> String {
        self.node.borrow().node_value.clone().unwrap_or_default()
    }

    pub fn set_data(&self, data: &str) {
        self.node.borrow_mut().node_value = Some(data.to_string());
    }

    pub fn length(&self) -> usize {
        self.get_data().len()
    }
}
