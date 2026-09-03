use crate::dom::node::Node;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub struct AttributeManager;

impl AttributeManager {
    pub fn new() -> Self {
        AttributeManager
    }

    pub fn get_attribute(node: &Rc<RefCell<Node>>, name: &str) -> Option<String> {
        node.borrow().get_attribute(name).cloned()
    }

    pub fn set_attribute(node: &Rc<RefCell<Node>>, name: &str, value: &str) {
        node.borrow_mut().set_attribute(name, value);
    }

    pub fn remove_attribute(node: &Rc<RefCell<Node>>, name: &str) -> Option<String> {
        node.borrow_mut().remove_attribute(name)
    }

    pub fn has_attribute(node: &Rc<RefCell<Node>>, name: &str) -> bool {
        node.borrow().has_attribute(name)
    }

    pub fn get_all_attributes(node: &Rc<RefCell<Node>>) -> HashMap<String, String> {
        node.borrow().attributes.clone()
    }

    pub fn get_attribute_names(node: &Rc<RefCell<Node>>) -> Vec<String> {
        node.borrow().attributes.keys().cloned().collect()
    }

    pub fn toggle_attribute(node: &Rc<RefCell<Node>>, name: &str) -> bool {
        let has = node.borrow().has_attribute(name);
        if has {
            node.borrow_mut().remove_attribute(name);
            false
        } else {
            node.borrow_mut().set_attribute(name, "");
            true
        }
    }

    pub fn set_attributes_batch(
        node: &Rc<RefCell<Node>>,
        attrs: &HashMap<String, String>,
    ) {
        let mut borrowed = node.borrow_mut();
        for (key, value) in attrs {
            borrowed.set_attribute(key, value);
        }
    }

    pub fn copy_attributes(
        source: &Rc<RefCell<Node>>,
        target: &Rc<RefCell<Node>>,
    ) {
        let attrs = source.borrow().attributes.clone();
        let mut borrowed = target.borrow_mut();
        for (key, value) in &attrs {
            borrowed.set_attribute(key, value);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dom::node::NodeType;

    fn make_node() -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node::new(NodeType::Element)))
    }

    #[test]
    fn test_new() {
        let _mgr = AttributeManager::new();
    }

    #[test]
    fn test_get_set_attribute() {
        let node = make_node();
        assert_eq!(AttributeManager::get_attribute(&node, "id"), None);
        AttributeManager::set_attribute(&node, "id", "test");
        assert_eq!(
            AttributeManager::get_attribute(&node, "id"),
            Some("test".to_string())
        );
    }

    #[test]
    fn test_remove_attribute() {
        let node = make_node();
        AttributeManager::set_attribute(&node, "role", "button");
        let removed = AttributeManager::remove_attribute(&node, "role");
        assert_eq!(removed, Some("button".to_string()));
        assert!(!AttributeManager::has_attribute(&node, "role"));
    }

    #[test]
    fn test_remove_nonexistent() {
        let node = make_node();
        let removed = AttributeManager::remove_attribute(&node, "missing");
        assert!(removed.is_none());
    }

    #[test]
    fn test_has_attribute() {
        let node = make_node();
        assert!(!AttributeManager::has_attribute(&node, "class"));
        AttributeManager::set_attribute(&node, "class", "box");
        assert!(AttributeManager::has_attribute(&node, "class"));
    }

    #[test]
    fn test_get_all_attributes() {
        let node = make_node();
        AttributeManager::set_attribute(&node, "id", "a");
        AttributeManager::set_attribute(&node, "class", "b");
        let all = AttributeManager::get_all_attributes(&node);
        assert_eq!(all.len(), 2);
        assert_eq!(all.get("id"), Some(&"a".to_string()));
        assert_eq!(all.get("class"), Some(&"b".to_string()));
    }

    #[test]
    fn test_get_attribute_names() {
        let node = make_node();
        AttributeManager::set_attribute(&node, "x", "1");
        AttributeManager::set_attribute(&node, "y", "2");
        let mut names = AttributeManager::get_attribute_names(&node);
        names.sort();
        assert_eq!(names, vec!["x", "y"]);
    }

    #[test]
    fn test_toggle_attribute_add() {
        let node = make_node();
        let result = AttributeManager::toggle_attribute(&node, "hidden");
        assert!(result);
        assert!(AttributeManager::has_attribute(&node, "hidden"));
    }

    #[test]
    fn test_toggle_attribute_remove() {
        let node = make_node();
        AttributeManager::set_attribute(&node, "hidden", "");
        let result = AttributeManager::toggle_attribute(&node, "hidden");
        assert!(!result);
        assert!(!AttributeManager::has_attribute(&node, "hidden"));
    }

    #[test]
    fn test_set_attributes_batch() {
        let node = make_node();
        let mut batch = HashMap::new();
        batch.insert("a".to_string(), "1".to_string());
        batch.insert("b".to_string(), "2".to_string());
        AttributeManager::set_attributes_batch(&node, &batch);
        assert_eq!(
            AttributeManager::get_attribute(&node, "a"),
            Some("1".to_string())
        );
        assert_eq!(
            AttributeManager::get_attribute(&node, "b"),
            Some("2".to_string())
        );
    }

    #[test]
    fn test_copy_attributes() {
        let src = make_node();
        let tgt = make_node();
        AttributeManager::set_attribute(&src, "role", "img");
        AttributeManager::set_attribute(&src, "aria-label", "icon");
        AttributeManager::copy_attributes(&src, &tgt);
        assert_eq!(
            AttributeManager::get_attribute(&tgt, "role"),
            Some("img".to_string())
        );
        assert_eq!(
            AttributeManager::get_attribute(&tgt, "aria-label"),
            Some("icon".to_string())
        );
    }

    #[test]
    fn test_overwrite_attribute() {
        let node = make_node();
        AttributeManager::set_attribute(&node, "val", "old");
        AttributeManager::set_attribute(&node, "val", "new");
        assert_eq!(
            AttributeManager::get_attribute(&node, "val"),
            Some("new".to_string())
        );
    }
}
