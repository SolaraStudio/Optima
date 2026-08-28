use super::node::{Node, NodeType, ElementData};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

pub struct Document {
    pub root: Arc<RwLock<Node>>,
    pub title: Option<String>,
    pub url: Option<String>,
    pub content_type: String,
    pub character_set: String,
    pub ready_state: DocumentReadyState,
}

#[derive(Debug, Clone, PartialEq)]
pub enum DocumentReadyState {
    Loading,
    Interactive,
    Complete,
}

impl Document {
    pub fn new() -> Self {
        let mut root = Node::new_element("html");
        let mut head = Node::new_element("head");
        let mut body = Node::new_element("body");
        root.children.push(Arc::new(RwLock::new(head)));
        root.children.push(Arc::new(RwLock::new(body)));
        Self {
            root: Arc::new(RwLock::new(root)),
            title: None,
            url: None,
            content_type: "text/html".to_string(),
            character_set: "UTF-8".to_string(),
            ready_state: DocumentReadyState::Loading,
        }
    }

    pub fn create_element(&self, tag: &str) -> Node {
        Node::new_element(tag)
    }

    pub fn create_text_node(&self, content: &str) -> Node {
        Node::new_text(content)
    }

    pub fn create_comment(&self, content: &str) -> Node {
        Node::new_comment(content)
    }

    pub fn create_document_fragment(&self) -> Node {
        Node::new_document_fragment()
    }

    pub fn get_element_by_id(&self, id: &str) -> Option<Node> {
        self.find_element(&self.root, |node| {
            if let Some(data) = &node.element_data {
                data.get_attribute("id") == Some(&id.to_string())
            } else {
                false
            }
        })
    }

    pub fn get_elements_by_tag_name(&self, tag: &str) -> Vec<Node> {
        let mut result = Vec::new();
        self.collect_elements(&self.root, tag, &mut result);
        result
    }

    pub fn get_elements_by_class_name(&self, class_name: &str) -> Vec<Node> {
        let mut result = Vec::new();
        self.collect_elements_by_class(&self.root, class_name, &mut result);
        result
    }

    pub fn query_selector(&self, selector: &str) -> Option<Node> {
        // Simplified selector matching
        let selector = selector.trim();
        if selector.starts_with('#') {
            return self.get_element_by_id(&selector[1..]);
        } else if selector.starts_with('.') {
            return self.get_elements_by_class_name(&selector[1..]).first().cloned();
        } else {
            return self.get_elements_by_tag_name(selector).first().cloned();
        }
    }

    pub fn query_selector_all(&self, selector: &str) -> Vec<Node> {
        let selector = selector.trim();
        if selector.starts_with('#') {
            if let Some(node) = self.get_element_by_id(&selector[1..]) {
                return vec![node];
            }
            return Vec::new();
        } else if selector.starts_with('.') {
            return self.get_elements_by_class_name(&selector[1..]);
        } else {
            return self.get_elements_by_tag_name(selector);
        }
    }

    pub fn get_body(&self) -> Option<Node> {
        // Find the first body element
        let root = self.root.read().unwrap();
        self.find_element(&self.root, |node| {
            if let Some(data) = &node.element_data {
                data.tag_name == "body"
            } else {
                false
            }
        })
    }

    pub fn get_head(&self) -> Option<Node> {
        self.find_element(&self.root, |node| {
            if let Some(data) = &node.element_data {
                data.tag_name == "head"
            } else {
                false
            }
        })
    }

    pub fn set_title(&mut self, title: &str) {
        self.title = Some(title.to_string());
    }

    pub fn set_url(&mut self, url: &str) {
        self.url = Some(url.to_string());
    }

    pub fn set_ready_state(&mut self, state: DocumentReadyState) {
        self.ready_state = state;
    }

    pub fn is_loading(&self) -> bool {
        matches!(self.ready_state, DocumentReadyState::Loading)
    }

    pub fn is_interactive(&self) -> bool {
        matches!(self.ready_state, DocumentReadyState::Interactive)
    }

    pub fn is_complete(&self) -> bool {
        matches!(self.ready_state, DocumentReadyState::Complete)
    }

    fn find_element<F>(&self, node: &Arc<RwLock<Node>>, predicate: F) -> Option<Node>
    where
        F: Fn(&Node) -> bool + Copy,
    {
        let node = node.read().unwrap();
        if predicate(&*node) {
            return Some(node.clone());
        }
        for child in &node.children {
            if let Some(found) = self.find_element(child, predicate) {
                return Some(found);
            }
        }
        None
    }

    fn collect_elements(&self, node: &Arc<RwLock<Node>>, tag: &str, result: &mut Vec<Node>) {
        let node = node.read().unwrap();
        if let Some(data) = &node.element_data {
            if data.tag_name == tag {
                result.push(node.clone());
            }
        }
        for child in &node.children {
            self.collect_elements(child, tag, result);
        }
    }

    fn collect_elements_by_class(&self, node: &Arc<RwLock<Node>>, class_name: &str, result: &mut Vec<Node>) {
        let node = node.read().unwrap();
        if let Some(data) = &node.element_data {
            if data.has_class(class_name) {
                result.push(node.clone());
            }
        }
        for child in &node.children {
            self.collect_elements_by_class(child, class_name, result);
        }
    }

    pub fn to_string(&self) -> String {
        let root = self.root.read().unwrap();
        format!("<!DOCTYPE html>\n{}", self.node_to_string(&root))
    }

    fn node_to_string(&self, node: &Node) -> String {
        match node.node_type {
            NodeType::Element => {
                if let Some(data) = &node.element_data {
                    let attr_str = data.attributes
                        .iter()
                        .map(|(k, v)| format!(" {}=\"{}\"", k, v))
                        .collect::<String>();
                    let children_str = node.children
                        .iter()
                        .map(|child| self.node_to_string(&child.read().unwrap()))
                        .collect::<String>();
                    format!("<{}{}>{}</{}>", data.tag_name, attr_str, children_str, data.tag_name)
                } else {
                    String::new()
                }
            }
            NodeType::Text => node.text_content.clone().unwrap_or_default(),
            NodeType::Comment => format!("<!--{}-->", node.text_content.clone().unwrap_or_default()),
            NodeType::Doctype => format!("<!DOCTYPE {}>", node.node_name),
            _ => String::new(),
        }
    }
}

impl Default for Document {
    fn default() -> Self {
        Self::new()
    }
}
