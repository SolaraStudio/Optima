use std::collections::HashMap;

pub enum NodeType {
    Element,
    Text,
    Comment,
    Document,
}

pub struct Node {
    pub node_type: NodeType,
    pub children: Vec<Node>,
    pub parent: Option<usize>,
    pub element_data: Option<ElementData>,
    pub text_content: Option<String>,
}

pub struct ElementData {
    pub tag_name: String,
    pub attributes: HashMap<String, String>,
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
            }),
            text_content: None,
        }
    }

    pub fn new_text(content: &str) -> Self {
        Self {
            node_type: NodeType::Text,
            children: Vec::new(),
            parent: None,
            element_data: None,
            text_content: Some(content.to_string()),
        }
    }
}
