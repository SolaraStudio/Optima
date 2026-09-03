use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct DOMNode {
    pub node_id: u64,
    pub node_type: u32,
    pub node_name: String,
    pub local_name: String,
    pub node_value: Option<String>,
    pub parent_id: Option<u64>,
    pub child_node_count: u32,
    pub children: Vec<DOMNode>,
    pub attributes: Vec<String>,
    pub document_url: Option<String>,
    pub base_url: Option<String>,
}

#[derive(Debug, Clone)]
pub struct DOMDocument {
    pub root: DOMNode,
    pub node_id_counter: u64,
}

pub struct DOMBackend {
    pub documents: HashMap<u64, DOMDocument>,
    pub next_document_id: u64,
}

impl DOMBackend {
    pub fn new() -> Self {
        let mut backend = DOMBackend {
            documents: HashMap::new(),
            next_document_id: 1,
        };
        backend.create_default_document();
        backend
    }

    fn create_default_document(&mut self) {
        let doc_id = self.next_document_id;
        self.next_document_id += 1;

        let root = DOMNode {
            node_id: 1,
            node_type: 9,
            node_name: "#document".to_string(),
            local_name: "".to_string(),
            node_value: None,
            parent_id: None,
            child_node_count: 2,
            children: Vec::new(),
            attributes: Vec::new(),
            document_url: Some("about:blank".to_string()),
            base_url: Some("about:blank".to_string()),
        };

        let _html = DOMNode {
            node_id: 2,
            node_type: 1,
            node_name: "HTML".to_string(),
            local_name: "html".to_string(),
            node_value: None,
            parent_id: Some(1),
            child_node_count: 2,
            children: Vec::new(),
            attributes: Vec::new(),
            document_url: None,
            base_url: None,
        };

        let _head = DOMNode {
            node_id: 3,
            node_type: 1,
            node_name: "HEAD".to_string(),
            local_name: "head".to_string(),
            node_value: None,
            parent_id: Some(2),
            child_node_count: 0,
            children: Vec::new(),
            attributes: Vec::new(),
            document_url: None,
            base_url: None,
        };

        let _body = DOMNode {
            node_id: 4,
            node_type: 1,
            node_name: "BODY".to_string(),
            local_name: "body".to_string(),
            node_value: None,
            parent_id: Some(2),
            child_node_count: 0,
            children: Vec::new(),
            attributes: Vec::new(),
            document_url: None,
            base_url: None,
        };

        let document = DOMDocument {
            root: root.clone(),
            node_id_counter: 4,
        };

        self.documents.insert(doc_id, document);
    }

    pub fn get_document(&self, doc_id: u64) -> Option<&DOMDocument> {
        self.documents.get(&doc_id)
    }

    pub fn get_document_mut(&mut self, doc_id: u64) -> Option<&mut DOMDocument> {
        self.documents.get_mut(&doc_id)
    }

    pub fn get_root_node(&self, doc_id: u64) -> Option<&DOMNode> {
        if let Some(doc) = self.documents.get(&doc_id) {
            Some(&doc.root)
        } else {
            None
        }
    }

    pub fn find_node(&self, doc_id: u64, node_id: u64) -> Option<&DOMNode> {
        if let Some(doc) = self.documents.get(&doc_id) {
            Self::find_node_recursive(&doc.root, node_id)
        } else {
            None
        }
    }

    fn find_node_recursive(node: &DOMNode, node_id: u64) -> Option<&DOMNode> {
        if node.node_id == node_id {
            return Some(node);
        }
        for child in &node.children {
            if let Some(found) = Self::find_node_recursive(child, node_id) {
                return Some(found);
            }
        }
        None
    }

    pub fn to_json(&self, doc_id: u64) -> Value {
        if let Some(doc) = self.documents.get(&doc_id) {
            Self::node_to_json(&doc.root)
        } else {
            serde_json::json!({})
        }
    }

    pub fn node_to_json(node: &DOMNode) -> Value {
        let children: Vec<Value> = node
            .children
            .iter()
            .map(|c| Self::node_to_json(c))
            .collect();
        serde_json::json!({
            "nodeId": node.node_id,
            "nodeType": node.node_type,
            "nodeName": node.node_name,
            "localName": node.local_name,
            "nodeValue": node.node_value,
            "childNodeCount": node.child_node_count,
            "children": children,
            "attributes": node.attributes,
            "documentURL": node.document_url,
            "baseURL": node.base_url
        })
    }
}

impl Default for DOMBackend {
    fn default() -> Self {
        Self::new()
    }
}
