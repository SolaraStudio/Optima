use std::collections::HashMap;
use std::sync::{Arc, RwLock};

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
    pub children: Vec<Arc<RwLock<Node>>>,
    pub parent: Option<Arc<RwLock<Node>>>,
    pub element_data: Option<ElementData>,
    pub text_content: Option<String>,
    pub node_name: String,
    pub node_value: Option<String>,
}

impl Node {
    pub fn new_element(tag: &str) -> Self {
        Self {
            node_type: NodeType::Element,
            children: Vec::new(),
            parent: None,
            element_data: Some(ElementData {
                tag_name: tag.to_string(),
                attributes: HashMap::new(),
                class_list: Vec::new(),
                style: HashMap::new(),
            }),
            text_content: None,
            node_name: tag.to_string(),
            node_value: None,
        }
    }

    pub fn new_text(content: &str) -> Self {
        Self {
            node_type: NodeType::Text,
            children: Vec::new(),
            parent: None,
            element_data: None,
            text_content: Some(content.to_string()),
            node_name: "#text".to_string(),
            node_value: Some(content.to_string()),
        }
    }

    pub fn new_comment(content: &str) -> Self {
        Self {
            node_type: NodeType::Comment,
            children: Vec::new(),
            parent: None,
            element_data: None,
            text_content: Some(content.to_string()),
            node_name: "#comment".to_string(),
            node_value: Some(content.to_string()),
        }
    }

    pub fn new_doctype(name: &str) -> Self {
        Self {
            node_type: NodeType::Doctype,
            children: Vec::new(),
            parent: None,
            element_data: None,
            text_content: None,
            node_name: name.to_string(),
            node_value: None,
        }
    }

    pub fn new_document_fragment() -> Self {
        Self {
            node_type: NodeType::DocumentFragment,
            children: Vec::new(),
            parent: None,
            element_data: None,
            text_content: None,
            node_name: "#document-fragment".to_string(),
            node_value: None,
        }
    }

    pub fn append_child(&mut self, child: Node) -> Arc<RwLock<Node>> {
        let child_arc = Arc::new(RwLock::new(child));
        self.children.push(child_arc.clone());
        if let Ok(mut child) = child_arc.write() {
            child.parent = Some(self.get_arc());
        }
        child_arc
    }

    pub fn remove_child(&mut self, index: usize) -> Option<Arc<RwLock<Node>>> {
        if index < self.children.len() {
            let child = self.children.remove(index);
            if let Ok(mut child) = child.write() {
                child.parent = None;
            }
            Some(child)
        } else {
            None
        }
    }

    pub fn get_children(&self) -> &Vec<Arc<RwLock<Node>>> {
        &self.children
    }

    pub fn get_parent(&self) -> Option<Arc<RwLock<Node>>> {
        self.parent.clone()
    }

    pub fn get_element_data(&self) -> Option<&ElementData> {
        self.element_data.as_ref()
    }

    pub fn get_element_data_mut(&mut self) -> Option<&mut ElementData> {
        self.element_data.as_mut()
    }

    pub fn get_text_content(&self) -> Option<&String> {
        self.text_content.as_ref()
    }

    pub fn get_node_name(&self) -> &str {
        &self.node_name
    }

    pub fn get_node_value(&self) -> Option<&String> {
        self.node_value.as_ref()
    }

    pub fn is_element(&self) -> bool {
        matches!(self.node_type, NodeType::Element)
    }

    pub fn is_text(&self) -> bool {
        matches!(self.node_type, NodeType::Text)
    }

    pub fn is_comment(&self) -> bool {
        matches!(self.node_type, NodeType::Comment)
    }

    pub fn is_document(&self) -> bool {
        matches!(self.node_type, NodeType::Document)
    }

    pub fn is_doctype(&self) -> bool {
        matches!(self.node_type, NodeType::Doctype)
    }

    pub fn is_fragment(&self) -> bool {
        matches!(self.node_type, NodeType::DocumentFragment)
    }

    fn get_arc(&self) -> Arc<RwLock<Node>> {
        // This is a simplified version; in practice you'd store the Arc elsewhere
        Arc::new(RwLock::new(self.clone()))
    }

    pub fn to_string(&self) -> String {
        match self.node_type {
            NodeType::Element => {
                if let Some(data) = &self.element_data {
                    let attr_str = data.attributes
                        .iter()
                        .map(|(k, v)| format!(" {}=\"{}\"", k, v))
                        .collect::<String>();
                    format!("<{}{}>", data.tag_name, attr_str)
                } else {
                    format!("<{}>", self.node_name)
                }
            }
            NodeType::Text => self.text_content.clone().unwrap_or_default(),
            NodeType::Comment => format!("<!--{}-->", self.text_content.clone().unwrap_or_default()),
            NodeType::Doctype => format!("<!DOCTYPE {}>", self.node_name),
            _ => self.node_name.clone(),
        }
    }
}

impl Default for Node {
    fn default() -> Self {
        Self {
            node_type: NodeType::Element,
            children: Vec::new(),
            parent: None,
            element_data: None,
            text_content: None,
            node_name: "".to_string(),
            node_value: None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ElementData {
    pub tag_name: String,
    pub attributes: HashMap<String, String>,
    pub class_list: Vec<String>,
    pub style: HashMap<String, String>,
}

impl ElementData {
    pub fn new(tag: &str) -> Self {
        Self {
            tag_name: tag.to_string(),
            attributes: HashMap::new(),
            class_list: Vec::new(),
            style: HashMap::new(),
        }
    }

    pub fn get_attribute(&self, key: &str) -> Option<&String> {
        self.attributes.get(key)
    }

    pub fn set_attribute(&mut self, key: &str, value: &str) {
        self.attributes.insert(key.to_string(), value.to_string());
    }

    pub fn remove_attribute(&mut self, key: &str) -> Option<String> {
        self.attributes.remove(key)
    }

    pub fn has_attribute(&self, key: &str) -> bool {
        self.attributes.contains_key(key)
    }

    pub fn get_id(&self) -> Option<&String> {
        self.get_attribute("id")
    }

    pub fn get_classes(&self) -> &Vec<String> {
        &self.class_list
    }

    pub fn has_class(&self, class_name: &str) -> bool {
        self.class_list.contains(&class_name.to_string())
    }

    pub fn add_class(&mut self, class_name: &str) {
        if !self.has_class(class_name) {
            self.class_list.push(class_name.to_string());
        }
    }

    pub fn remove_class(&mut self, class_name: &str) {
        self.class_list.retain(|c| c != class_name);
    }

    pub fn toggle_class(&mut self, class_name: &str) -> bool {
        if self.has_class(class_name) {
            self.remove_class(class_name);
            false
        } else {
            self.add_class(class_name);
            true
        }
    }

    pub fn get_style(&self, key: &str) -> Option<&String> {
        self.style.get(key)
    }

    pub fn set_style(&mut self, key: &str, value: &str) {
        self.style.insert(key.to_string(), value.to_string());
    }

    pub fn remove_style(&mut self, key: &str) -> Option<String> {
        self.style.remove(key)
    }
}
