use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq)]
pub enum NodeType {
    Document,
    Element,
    Text,
    Comment,
    Doctype,
    DocumentFragment,
}

#[derive(Debug, Clone)]
pub struct Node {
    pub node_type: NodeType,
    pub node_name: String,
    pub node_value: Option<String>,
    pub parent: Option<Rc<RefCell<Node>>>,
    pub children: Vec<Rc<RefCell<Node>>>,
    pub attributes: HashMap<String, String>,
    pub tag_name: Option<String>,
}

impl Node {
    pub fn new(node_type: NodeType) -> Self {
        let node_name = match node_type {
            NodeType::Document => "#document".to_string(),
            NodeType::Element => "".to_string(),
            NodeType::Text => "#text".to_string(),
            NodeType::Comment => "#comment".to_string(),
            NodeType::Doctype => "html".to_string(),
            NodeType::DocumentFragment => "#document-fragment".to_string(),
        };
        Node {
            node_type,
            node_name,
            node_value: None,
            parent: None,
            children: Vec::new(),
            attributes: HashMap::new(),
            tag_name: None,
        }
    }

    pub fn create_element(tag: &str) -> Self {
        let mut node = Node::new(NodeType::Element);
        node.node_name = tag.to_string();
        node.tag_name = Some(tag.to_string());
        node
    }

    pub fn create_text(text: &str) -> Self {
        let mut node = Node::new(NodeType::Text);
        node.node_value = Some(text.to_string());
        node
    }

    pub fn create_comment(text: &str) -> Self {
        let mut node = Node::new(NodeType::Comment);
        node.node_value = Some(text.to_string());
        node
    }

    pub fn append_child(&mut self, child: Rc<RefCell<Node>>) -> Rc<RefCell<Node>> {
        child.borrow_mut().parent = Some(Rc::clone(&child));
        self.children.push(Rc::clone(&child));
        child
    }

    pub fn remove_child(&mut self, child: &Rc<RefCell<Node>>) -> Option<Rc<RefCell<Node>>> {
        if let Some(pos) = self.children.iter().position(|c| Rc::ptr_eq(c, child)) {
            let removed = self.children.remove(pos);
            removed.borrow_mut().parent = None;
            Some(removed)
        } else {
            None
        }
    }

    pub fn first_child(&self) -> Option<Rc<RefCell<Node>>> {
        self.children.first().cloned()
    }

    pub fn last_child(&self) -> Option<Rc<RefCell<Node>>> {
        self.children.last().cloned()
    }

    pub fn parent_node(&self) -> Option<Rc<RefCell<Node>>> {
        self.parent.clone()
    }

    pub fn child_nodes(&self) -> Vec<Rc<RefCell<Node>>> {
        self.children.clone()
    }

    pub fn has_child_nodes(&self) -> bool {
        !self.children.is_empty()
    }

    pub fn get_attribute(&self, name: &str) -> Option<&String> {
        self.attributes.get(name)
    }

    pub fn set_attribute(&mut self, name: &str, value: &str) {
        self.attributes.insert(name.to_string(), value.to_string());
    }

    pub fn remove_attribute(&mut self, name: &str) -> Option<String> {
        self.attributes.remove(name)
    }

    pub fn has_attribute(&self, name: &str) -> bool {
        self.attributes.contains_key(name)
    }

    pub fn is_element_node(&self) -> bool {
        self.node_type == NodeType::Element
    }

    pub fn is_text_node(&self) -> bool {
        self.node_type == NodeType::Text
    }

    pub fn is_document_node(&self) -> bool {
        self.node_type == NodeType::Document
    }

    pub fn clone_node(&self, deep: bool) -> Self {
        let mut clone = Node {
            node_type: self.node_type.clone(),
            node_name: self.node_name.clone(),
            node_value: self.node_value.clone(),
            parent: None,
            children: Vec::new(),
            attributes: self.attributes.clone(),
            tag_name: self.tag_name.clone(),
        };
        if deep {
            for child in &self.children {
                let child_clone = child.borrow().clone_node(true);
                clone.children.push(Rc::new(RefCell::new(child_clone)));
            }
        }
        clone
    }
}
