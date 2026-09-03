use crate::dom::comment::Comment;
use crate::dom::doctype::Doctype;
use crate::dom::element::Element;
use crate::dom::node::{Node, NodeType};
use crate::dom::text::Text;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Document {
    pub node: Rc<RefCell<Node>>,
}

impl Default for Document {
    fn default() -> Self {
        Self::new()
    }
}

impl Document {
    pub fn new() -> Self {
        let node = Rc::new(RefCell::new(Node::new(NodeType::Document)));
        Document { node }
    }

    pub fn create_element(&self, tag: &str) -> Element {
        Element::new(tag)
    }

    pub fn create_text_node(&self, text: &str) -> Text {
        Text::new(text)
    }

    pub fn create_comment(&self, text: &str) -> Comment {
        Comment::new(text)
    }

    pub fn create_doctype(&self, name: &str) -> Doctype {
        Doctype::new(name)
    }

    pub fn get_body(&self) -> Option<Element> {
        self.find_element_by_tag("body")
    }

    pub fn get_head(&self) -> Option<Element> {
        self.find_element_by_tag("head")
    }

    pub fn get_element_by_id(&self, id: &str) -> Option<Element> {
        self.find_element_by_attribute("id", id)
    }

    pub fn get_elements_by_tag_name(&self, tag: &str) -> Vec<Element> {
        let mut elements = Vec::new();
        self.collect_elements_by_tag(&self.node, tag, &mut elements);
        elements
    }

    pub fn get_elements_by_class_name(&self, class_name: &str) -> Vec<Element> {
        let mut elements = Vec::new();
        self.collect_elements_by_class(&self.node, class_name, &mut elements);
        elements
    }

    pub fn query_selector(&self, selector: &str) -> Option<Element> {
        if let Some(id) = selector.strip_prefix('#') {
            return self.get_element_by_id(id);
        }
        if let Some(class) = selector.strip_prefix('.') {
            let elements = self.get_elements_by_class_name(class);
            return elements.first().cloned();
        }
        let elements = self.get_elements_by_tag_name(selector);
        elements.first().cloned()
    }

    pub fn query_selector_all(&self, selector: &str) -> Vec<Element> {
        if let Some(id) = selector.strip_prefix('#') {
            if let Some(el) = self.get_element_by_id(id) {
                return vec![el];
            }
            return Vec::new();
        }
        if let Some(class) = selector.strip_prefix('.') {
            return self.get_elements_by_class_name(class);
        }
        self.get_elements_by_tag_name(selector)
    }

    fn find_element_by_tag(&self, tag: &str) -> Option<Element> {
        self.find_element(&self.node, |node| {
            if let Some(tag_name) = &node.borrow().tag_name {
                tag_name == tag
            } else {
                false
            }
        })
    }

    fn find_element_by_attribute(&self, attr: &str, value: &str) -> Option<Element> {
        self.find_element(&self.node, |node| {
            node.borrow().get_attribute(attr) == Some(&value.to_string())
        })
    }

    fn find_element<F>(&self, node: &Rc<RefCell<Node>>, predicate: F) -> Option<Element>
    where
        F: Fn(&Rc<RefCell<Node>>) -> bool + Copy,
    {
        if predicate(node) {
            return Some(Element::from_node(Rc::clone(node)));
        }
        for child in &node.borrow().children {
            if let Some(found) = self.find_element(child, predicate) {
                return Some(found);
            }
        }
        None
    }

    fn collect_elements_by_tag(
        &self,
        node: &Rc<RefCell<Node>>,
        tag: &str,
        result: &mut Vec<Element>,
    ) {
        if let Some(tag_name) = &node.borrow().tag_name
            && tag_name == tag {
                result.push(Element::from_node(Rc::clone(node)));
            }
        for child in &node.borrow().children {
            self.collect_elements_by_tag(child, tag, result);
        }
    }

    fn collect_elements_by_class(
        &self,
        node: &Rc<RefCell<Node>>,
        class_name: &str,
        result: &mut Vec<Element>,
    ) {
        if let Some(class_attr) = node.borrow().get_attribute("class")
            && class_attr.split_whitespace().any(|c| c == class_name) {
                result.push(Element::from_node(Rc::clone(node)));
            }
        for child in &node.borrow().children {
            self.collect_elements_by_class(child, class_name, result);
        }
    }

    pub fn to_string(&self) -> String {
        let mut html = String::new();
        for child in &self.node.borrow().children {
            html.push_str(&Self::node_to_string(child));
        }
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
            crate::dom::node::NodeType::Doctype => {
                format!("<!DOCTYPE {}>", node.node_name)
            }
            _ => String::new(),
        }
    }
}
