use crate::dom::node::Node;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone)]
pub struct Element {
    pub node: Rc<RefCell<Node>>,
}

impl Element {
    pub fn new(tag: &str) -> Self {
        let node = Rc::new(RefCell::new(Node::create_element(tag)));
        Element { node }
    }

    pub fn from_node(node: Rc<RefCell<Node>>) -> Self {
        Element { node }
    }

    pub fn get_attribute(&self, name: &str) -> Option<String> {
        self.node.borrow().get_attribute(name).cloned()
    }

    pub fn set_attribute(&self, name: &str, value: &str) {
        self.node.borrow_mut().set_attribute(name, value);
    }

    pub fn remove_attribute(&self, name: &str) -> Option<String> {
        self.node.borrow_mut().remove_attribute(name)
    }

    pub fn has_attribute(&self, name: &str) -> bool {
        self.node.borrow().has_attribute(name)
    }

    pub fn tag_name(&self) -> String {
        self.node.borrow().node_name.clone()
    }

    pub fn inner_html(&self) -> String {
        let mut html = String::new();
        for child in self.node.borrow().children.clone() {
            html.push_str(&Self::node_to_string(&child));
        }
        html
    }

    pub fn outer_html(&self) -> String {
        let node = self.node.borrow();
        let mut html = String::new();
        html.push_str(&format!("<{}", node.node_name));
        for (key, value) in &node.attributes {
            html.push_str(&format!(" {}=\"{}\"", key, value));
        }
        html.push('>');
        html.push_str(&self.inner_html());
        html.push_str(&format!("</{}>", node.node_name));
        html
    }

    fn node_to_string(node: &Rc<RefCell<Node>>) -> String {
        let node = node.borrow();
        match node.node_type {
            crate::dom::node::NodeType::Element => {
                let mut html = String::new();
                html.push_str(&format!("<{}", node.node_name));
                for (key, value) in &node.attributes {
                    html.push_str(&format!(" {}=\"{}\"", key, value));
                }
                html.push('>');
                for child in &node.children {
                    html.push_str(&Self::node_to_string(child));
                }
                html.push_str(&format!("</{}>", node.node_name));
                html
            }
            crate::dom::node::NodeType::Text => node.node_value.clone().unwrap_or_default(),
            crate::dom::node::NodeType::Comment => {
                format!("<!--{}-->", node.node_value.clone().unwrap_or_default())
            }
            _ => String::new(),
        }
    }

    pub fn set_inner_html(&self, html: &str) {
        // Simplified: clear children and parse HTML
        self.node.borrow_mut().children.clear();
        // In a real implementation, you would parse HTML here
        // For now, we just add a text node with the HTML as a placeholder
        let text_node = Rc::new(RefCell::new(Node::create_text(html)));
        self.node.borrow_mut().append_child(text_node);
    }
}
