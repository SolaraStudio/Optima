use crate::dom::node::Node;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Text {
    pub node: Rc<RefCell<Node>>,
}

impl Text {
    pub fn new(text: &str) -> Self {
        let node = Rc::new(RefCell::new(Node::create_text(text)));
        Text { node }
    }

    pub fn from_node(node: Rc<RefCell<Node>>) -> Self {
        Text { node }
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

    pub fn split_text(&self, offset: usize) -> Option<Text> {
        let data = self.get_data();
        if offset > data.len() {
            return None;
        }
        let left = &data[0..offset];
        let right = &data[offset..];
        self.set_data(left);
        Some(Text::new(right))
    }

    pub fn append_data(&self, data: &str) {
        let current = self.get_data();
        self.set_data(&format!("{}{}", current, data));
    }

    pub fn delete_data(&self, offset: usize, count: usize) {
        let mut data = self.get_data();
        if offset < data.len() {
            let end = (offset + count).min(data.len());
            data.replace_range(offset..end, "");
            self.set_data(&data);
        }
    }

    pub fn insert_data(&self, offset: usize, data: &str) {
        let mut current = self.get_data();
        if offset <= current.len() {
            current.insert_str(offset, data);
            self.set_data(&current);
        }
    }
}
