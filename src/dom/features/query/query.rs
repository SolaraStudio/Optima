use crate::dom::node::Node;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq)]
pub enum SelectorKind {
    Tag(String),
    Id(String),
    Class(String),
}

pub struct QueryBuilder;

impl Default for QueryBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl QueryBuilder {
    pub fn new() -> Self {
        QueryBuilder
    }

    pub fn parse_selector(selector: &str) -> SelectorKind {
        if let Some(id) = selector.strip_prefix('#') {
            SelectorKind::Id(id.to_string())
        } else if let Some(cls) = selector.strip_prefix('.') {
            SelectorKind::Class(cls.to_string())
        } else {
            SelectorKind::Tag(selector.to_string())
        }
    }

    pub fn matches(node: &Rc<RefCell<Node>>, selector: &str) -> bool {
        let kind = Self::parse_selector(selector);
        let borrowed = node.borrow();
        match kind {
            SelectorKind::Tag(ref tag) => borrowed.tag_name.as_deref() == Some(tag.as_str()),
            SelectorKind::Id(ref id) => {
                borrowed.get_attribute("id").map(|s| s.as_str()) == Some(id.as_str())
            }
            SelectorKind::Class(ref cls) => match borrowed.get_attribute("class") {
                Some(class_attr) => class_attr.split_whitespace().any(|c| c == cls.as_str()),
                None => false,
            },
        }
    }

    pub fn query_selector(root: &Rc<RefCell<Node>>, selector: &str) -> Option<Rc<RefCell<Node>>> {
        Self::matches(root, selector)
            .then(|| Rc::clone(root))
            .or_else(|| {
                let children = root.borrow().children.clone();
                for child in &children {
                    if let Some(found) = Self::query_selector(child, selector) {
                        return Some(found);
                    }
                }
                None
            })
    }

    pub fn query_selector_all(root: &Rc<RefCell<Node>>, selector: &str) -> Vec<Rc<RefCell<Node>>> {
        let mut results = Vec::new();
        Self::collect_all(root, selector, &mut results);
        results
    }

    fn collect_all(node: &Rc<RefCell<Node>>, selector: &str, results: &mut Vec<Rc<RefCell<Node>>>) {
        if Self::matches(node, selector) {
            results.push(Rc::clone(node));
        }
        let children = node.borrow().children.clone();
        for child in &children {
            Self::collect_all(child, selector, results);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_element(tag: &str) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node::create_element(tag)))
    }

    #[test]
    fn test_parse_tag_selector() {
        let kind = QueryBuilder::parse_selector("div");
        assert_eq!(kind, SelectorKind::Tag("div".to_string()));
    }

    #[test]
    fn test_parse_id_selector() {
        let kind = QueryBuilder::parse_selector("#main");
        assert_eq!(kind, SelectorKind::Id("main".to_string()));
    }

    #[test]
    fn test_parse_class_selector() {
        let kind = QueryBuilder::parse_selector(".highlight");
        assert_eq!(kind, SelectorKind::Class("highlight".to_string()));
    }

    #[test]
    fn test_matches_tag() {
        let node = make_element("div");
        assert!(QueryBuilder::matches(&node, "div"));
        assert!(!QueryBuilder::matches(&node, "span"));
    }

    #[test]
    fn test_matches_id() {
        let node = make_element("div");
        node.borrow_mut().set_attribute("id", "main");
        assert!(QueryBuilder::matches(&node, "#main"));
        assert!(!QueryBuilder::matches(&node, "#other"));
    }

    #[test]
    fn test_matches_class() {
        let node = make_element("div");
        node.borrow_mut().set_attribute("class", "highlight active");
        assert!(QueryBuilder::matches(&node, ".highlight"));
        assert!(QueryBuilder::matches(&node, ".active"));
        assert!(!QueryBuilder::matches(&node, ".inactive"));
    }

    #[test]
    fn test_query_selector_by_tag() {
        let root = make_element("div");
        let child = make_element("span");
        root.borrow_mut().children.push(Rc::clone(&child));
        child.borrow_mut().parent = Some(Rc::clone(&root));

        let found = QueryBuilder::query_selector(&root, "span");
        assert!(found.is_some());
        assert_eq!(found.unwrap().borrow().node_name, "span");
    }

    #[test]
    fn test_query_selector_not_found() {
        let root = make_element("div");
        let found = QueryBuilder::query_selector(&root, "span");
        assert!(found.is_none());
    }

    #[test]
    fn test_query_selector_by_id() {
        let root = make_element("div");
        let child = make_element("p");
        child.borrow_mut().set_attribute("id", "intro");
        root.borrow_mut().children.push(Rc::clone(&child));
        child.borrow_mut().parent = Some(Rc::clone(&root));

        let found = QueryBuilder::query_selector(&root, "#intro");
        assert!(found.is_some());
    }

    #[test]
    fn test_query_selector_deep() {
        let root = make_element("div");
        let level1 = make_element("section");
        let level2 = make_element("p");
        level2.borrow_mut().set_attribute("id", "deep");
        level1.borrow_mut().children.push(Rc::clone(&level2));
        level2.borrow_mut().parent = Some(Rc::clone(&level1));
        root.borrow_mut().children.push(Rc::clone(&level1));
        level1.borrow_mut().parent = Some(Rc::clone(&root));

        let found = QueryBuilder::query_selector(&root, "#deep");
        assert!(found.is_some());
        assert_eq!(found.unwrap().borrow().node_name, "p");
    }

    #[test]
    fn test_query_selector_all() {
        let root = make_element("div");
        let c1 = make_element("span");
        let c2 = make_element("span");
        let c3 = make_element("p");
        root.borrow_mut().children.push(Rc::clone(&c1));
        root.borrow_mut().children.push(Rc::clone(&c2));
        root.borrow_mut().children.push(Rc::clone(&c3));

        let results = QueryBuilder::query_selector_all(&root, "span");
        assert_eq!(results.len(), 2);
    }

    #[test]
    fn test_query_selector_all_by_class() {
        let root = make_element("div");
        let c1 = make_element("p");
        c1.borrow_mut().set_attribute("class", "item");
        let c2 = make_element("div");
        c2.borrow_mut().set_attribute("class", "item");
        let c3 = make_element("span");
        c3.borrow_mut().set_attribute("class", "other");
        root.borrow_mut().children.push(Rc::clone(&c1));
        root.borrow_mut().children.push(Rc::clone(&c2));
        root.borrow_mut().children.push(Rc::clone(&c3));

        let results = QueryBuilder::query_selector_all(&root, ".item");
        assert_eq!(results.len(), 2);
    }

    #[test]
    fn test_query_selector_first_match() {
        let root = make_element("div");
        let c1 = make_element("span");
        c1.borrow_mut().set_attribute("class", "a");
        let c2 = make_element("span");
        c2.borrow_mut().set_attribute("class", "a");
        root.borrow_mut().children.push(Rc::clone(&c1));
        root.borrow_mut().children.push(Rc::clone(&c2));

        let found = QueryBuilder::query_selector(&root, ".a");
        assert!(found.is_some());
        assert!(Rc::ptr_eq(&found.unwrap(), &c1));
    }

    #[test]
    fn test_new() {
        let _qb = QueryBuilder::new();
    }
}
