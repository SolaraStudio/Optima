use super::node::{Node, NodeType, ElementData};
use std::collections::HashMap;

pub struct Document {
    pub root: Node,
    pub title: Option<String>,
}

impl Document {
    pub fn new() -> Self {
        let mut root = Node::new_element("html");
        let mut head = Node::new_element("head");
        let mut body = Node::new_element("body");
        root.children.push(head);
        root.children.push(body);
        Self {
            root,
            title: None,
        }
    }

    pub fn get_element_by_id(&self, id: &str) -> Option<&Node> {
        self.find_element(&self.root, |el| {
            el.element_data.as_ref().and_then(|d| d.attributes.get("id")) == Some(&id.to_string())
        })
    }

    pub fn get_elements_by_tag_name(&self, tag: &str) -> Vec<&Node> {
        let mut result = Vec::new();
        self.collect_elements(&self.root, tag, &mut result);
        result
    }

    fn find_element<F>(&self, node: &Node, predicate: F) -> Option<&Node>
    where
        F: Fn(&Node) -> bool,
    {
        if predicate(node) {
            return Some(node);
        }
        for child in &node.children {
            if let Some(found) = self.find_element(child, &predicate) {
                return Some(found);
            }
        }
        None
    }

    fn collect_elements(&self, node: &Node, tag: &str, result: &mut Vec<&Node>) {
        if let Some(data) = &node.element_data {
            if data.tag_name == tag {
                result.push(node);
            }
        }
        for child in &node.children {
            self.collect_elements(child, tag, result);
        }
    }
}
