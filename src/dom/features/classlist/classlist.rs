use crate::dom::node::Node;
use std::cell::RefCell;
use std::rc::Rc;

pub struct ClassList {
    node: Rc<RefCell<Node>>,
}

impl ClassList {
    pub fn new(node: Rc<RefCell<Node>>) -> Self {
        ClassList { node }
    }

    pub fn add(&self, class: &str) {
        if !self.contains(class) {
            let existing = self
                .node
                .borrow()
                .get_attribute("class")
                .cloned()
                .unwrap_or_default();
            if existing.is_empty() {
                self.node
                    .borrow_mut()
                    .set_attribute("class", class);
            } else {
                self.node
                    .borrow_mut()
                    .set_attribute("class", &format!("{} {}", existing, class));
            }
        }
    }

    pub fn remove(&self, class: &str) {
        let classes = self.to_vec();
        let filtered: Vec<&str> = classes
            .iter()
            .map(|s| s.as_str())
            .filter(|c| *c != class)
            .collect();
        let new_value = filtered.join(" ");
        self.node
            .borrow_mut()
            .set_attribute("class", &new_value);
    }

    pub fn toggle(&self, class: &str) -> bool {
        if self.contains(class) {
            self.remove(class);
            false
        } else {
            self.add(class);
            true
        }
    }

    pub fn contains(&self, class: &str) -> bool {
        self.to_vec().iter().any(|c| c == class)
    }

    pub fn item(&self, index: usize) -> Option<String> {
        self.to_vec().get(index).cloned()
    }

    pub fn length(&self) -> usize {
        self.to_vec().len()
    }

    pub fn value(&self) -> String {
        self.node
            .borrow()
            .get_attribute("class")
            .cloned()
            .unwrap_or_default()
    }

    pub fn set_value(&self, value: &str) {
        self.node
            .borrow_mut()
            .set_attribute("class", value);
    }

    pub fn to_vec(&self) -> Vec<String> {
        self.node
            .borrow()
            .get_attribute("class")
            .map(|v| {
                v.split_whitespace()
                    .map(|s| s.to_string())
                    .collect()
            })
            .unwrap_or_default()
    }

    pub fn replace(&self, old_class: &str, new_class: &str) -> bool {
        if self.contains(old_class) {
            self.remove(old_class);
            self.add(new_class);
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dom::node::NodeType;

    fn make_element() -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node::new(NodeType::Element)))
    }

    #[test]
    fn test_new() {
        let node = make_element();
        let _cl = ClassList::new(node);
    }

    #[test]
    fn test_add_single() {
        let node = make_element();
        let cl = ClassList::new(Rc::clone(&node));
        cl.add("foo");
        assert_eq!(cl.value(), "foo");
    }

    #[test]
    fn test_add_existing() {
        let node = make_element();
        let cl = ClassList::new(Rc::clone(&node));
        cl.add("a");
        cl.add("b");
        cl.add("a");
        assert_eq!(cl.value(), "a b");
    }

    #[test]
    fn test_remove() {
        let node = make_element();
        node.borrow_mut().set_attribute("class", "a b c");
        let cl = ClassList::new(Rc::clone(&node));
        cl.remove("b");
        assert_eq!(cl.value(), "a c");
    }

    #[test]
    fn test_remove_first() {
        let node = make_element();
        node.borrow_mut().set_attribute("class", "a b");
        let cl = ClassList::new(Rc::clone(&node));
        cl.remove("a");
        assert_eq!(cl.value(), "b");
    }

    #[test]
    fn test_remove_last() {
        let node = make_element();
        node.borrow_mut().set_attribute("class", "a b");
        let cl = ClassList::new(Rc::clone(&node));
        cl.remove("b");
        assert_eq!(cl.value(), "a");
    }

    #[test]
    fn test_toggle_add() {
        let node = make_element();
        let cl = ClassList::new(Rc::clone(&node));
        let result = cl.toggle("x");
        assert!(result);
        assert!(cl.contains("x"));
    }

    #[test]
    fn test_toggle_remove() {
        let node = make_element();
        node.borrow_mut().set_attribute("class", "x");
        let cl = ClassList::new(Rc::clone(&node));
        let result = cl.toggle("x");
        assert!(!result);
        assert!(!cl.contains("x"));
    }

    #[test]
    fn test_contains() {
        let node = make_element();
        node.borrow_mut().set_attribute("class", "red blue");
        let cl = ClassList::new(Rc::clone(&node));
        assert!(cl.contains("red"));
        assert!(cl.contains("blue"));
        assert!(!cl.contains("green"));
    }

    #[test]
    fn test_item() {
        let node = make_element();
        node.borrow_mut().set_attribute("class", "a b c");
        let cl = ClassList::new(Rc::clone(&node));
        assert_eq!(cl.item(0), Some("a".to_string()));
        assert_eq!(cl.item(1), Some("b".to_string()));
        assert_eq!(cl.item(2), Some("c".to_string()));
        assert_eq!(cl.item(3), None);
    }

    #[test]
    fn test_length() {
        let node = make_element();
        let cl = ClassList::new(Rc::clone(&node));
        assert_eq!(cl.length(), 0);
        cl.add("a");
        assert_eq!(cl.length(), 1);
        cl.add("b");
        assert_eq!(cl.length(), 2);
    }

    #[test]
    fn test_value() {
        let node = make_element();
        node.borrow_mut().set_attribute("class", "x y");
        let cl = ClassList::new(Rc::clone(&node));
        assert_eq!(cl.value(), "x y");
    }

    #[test]
    fn test_set_value() {
        let node = make_element();
        let cl = ClassList::new(Rc::clone(&node));
        cl.set_value("p q");
        assert_eq!(cl.value(), "p q");
    }

    #[test]
    fn test_to_vec() {
        let node = make_element();
        node.borrow_mut().set_attribute("class", "a b c");
        let cl = ClassList::new(Rc::clone(&node));
        let vec = cl.to_vec();
        assert_eq!(vec, vec!["a", "b", "c"]);
    }

    #[test]
    fn test_to_vec_empty() {
        let node = make_element();
        let cl = ClassList::new(Rc::clone(&node));
        assert!(cl.to_vec().is_empty());
    }

    #[test]
    fn test_replace() {
        let node = make_element();
        node.borrow_mut().set_attribute("class", "old other");
        let cl = ClassList::new(Rc::clone(&node));
        let replaced = cl.replace("old", "new");
        assert!(replaced);
        assert!(cl.contains("new"));
        assert!(!cl.contains("old"));
        assert!(cl.contains("other"));
    }

    #[test]
    fn test_replace_not_found() {
        let node = make_element();
        node.borrow_mut().set_attribute("class", "a b");
        let cl = ClassList::new(Rc::clone(&node));
        let replaced = cl.replace("z", "y");
        assert!(!replaced);
        assert_eq!(cl.value(), "a b");
    }
}
