use crate::dom::node::Node;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct MutationRecord {
    pub type_: String,
    pub target: Rc<RefCell<Node>>,
    pub added_nodes: Vec<Rc<RefCell<Node>>>,
    pub removed_nodes: Vec<Rc<RefCell<Node>>>,
    pub previous_sibling: Option<Rc<RefCell<Node>>>,
    pub next_sibling: Option<Rc<RefCell<Node>>>,
    pub attribute_name: Option<String>,
    pub old_value: Option<String>,
}

impl MutationRecord {
    pub fn new(type_: &str, target: Rc<RefCell<Node>>) -> Self {
        MutationRecord {
            type_: type_.to_string(),
            target,
            added_nodes: Vec::new(),
            removed_nodes: Vec::new(),
            previous_sibling: None,
            next_sibling: None,
            attribute_name: None,
            old_value: None,
        }
    }

    pub fn with_added_nodes(mut self, nodes: Vec<Rc<RefCell<Node>>>) -> Self {
        self.added_nodes = nodes;
        self
    }

    pub fn with_removed_nodes(mut self, nodes: Vec<Rc<RefCell<Node>>>) -> Self {
        self.removed_nodes = nodes;
        self
    }

    pub fn with_attribute(mut self, name: &str, old_value: &str) -> Self {
        self.attribute_name = Some(name.to_string());
        self.old_value = Some(old_value.to_string());
        self
    }
}
