use crate::dom::node::{Node, NodeType};
use std::cell::RefCell;
use std::rc::Rc;

pub struct HtmlSerializer;

impl Default for HtmlSerializer {
    fn default() -> Self {
        Self::new()
    }
}

impl HtmlSerializer {
    pub fn new() -> Self {
        HtmlSerializer
    }

    pub fn inner_html(node: &Rc<RefCell<Node>>) -> String {
        let borrowed = node.borrow();
        let mut html = String::new();
        for child in &borrowed.children {
            html.push_str(&Self::node_to_html(child));
        }
        html
    }

    pub fn outer_html(node: &Rc<RefCell<Node>>) -> String {
        let borrowed = node.borrow();
        match borrowed.node_type {
            NodeType::Element => {
                let mut html = format!("<{}", borrowed.node_name);
                Self::append_attrs(&mut html, &borrowed);
                if Self::is_void_element(&borrowed.node_name) {
                    html.push('>');
                } else {
                    html.push('>');
                    for child in &borrowed.children {
                        html.push_str(&Self::node_to_html(child));
                    }
                    html.push_str(&format!("</{}>", borrowed.node_name));
                }
                html
            }
            NodeType::Text => {
                let text = borrowed.node_value.clone().unwrap_or_default();
                Self::escape_text(&text)
            }
            NodeType::Comment => {
                let text = borrowed.node_value.clone().unwrap_or_default();
                format!("<!--{}-->", text)
            }
            NodeType::Doctype => format!("<!DOCTYPE {}>", borrowed.node_name),
            NodeType::Document => {
                let mut html = String::new();
                for child in &borrowed.children {
                    html.push_str(&Self::node_to_html(child));
                }
                html
            }
            NodeType::DocumentFragment => {
                let mut html = String::new();
                for child in &borrowed.children {
                    html.push_str(&Self::node_to_html(child));
                }
                html
            }
        }
    }

    pub fn set_inner_html(node: &Rc<RefCell<Node>>, html: &str) {
        {
            node.borrow_mut().children.clear();
        }
        let text_node = Rc::new(RefCell::new(Node::create_text(html)));
        node.borrow_mut().append_child(text_node);
    }

    pub fn set_outer_html(node: &Rc<RefCell<Node>>, html: &str) {
        if let Some(parent) = node.borrow().parent.clone() {
            let pos = parent
                .borrow()
                .children
                .iter()
                .position(|c| Rc::ptr_eq(c, node));
            if let Some(idx) = pos {
                let text_node = Rc::new(RefCell::new(Node::create_text(html)));
                parent.borrow_mut().children[idx] = Rc::clone(&text_node);
                text_node.borrow_mut().parent = Some(Rc::clone(&parent));
            }
        }
    }

    pub fn text_content(node: &Rc<RefCell<Node>>) -> String {
        let borrowed = node.borrow();
        match borrowed.node_type {
            NodeType::Text | NodeType::Comment => borrowed.node_value.clone().unwrap_or_default(),
            _ => {
                let mut text = String::new();
                for child in &borrowed.children {
                    text.push_str(&Self::text_content(child));
                }
                text
            }
        }
    }

    fn node_to_html(node: &Rc<RefCell<Node>>) -> String {
        Self::outer_html(node)
    }

    fn append_attrs(html: &mut String, node: &Node) {
        let mut sorted: Vec<_> = node.attributes.iter().collect();
        sorted.sort_by_key(|(k, _)| (*k).clone());
        for (key, value) in sorted {
            if value.is_empty() {
                html.push_str(&format!(" {}", key));
            } else {
                html.push_str(&format!(" {}=\"{}\"", key, Self::escape_attr(value)));
            }
        }
    }

    fn is_void_element(tag: &str) -> bool {
        matches!(
            tag,
            "area"
                | "base"
                | "br"
                | "col"
                | "embed"
                | "hr"
                | "img"
                | "input"
                | "link"
                | "meta"
                | "param"
                | "source"
                | "track"
                | "wbr"
        )
    }

    fn escape_text(text: &str) -> String {
        text.replace('&', "&amp;")
            .replace('<', "&lt;")
            .replace('>', "&gt;")
    }

    fn escape_attr(text: &str) -> String {
        text.replace('&', "&amp;")
            .replace('"', "&quot;")
            .replace('<', "&lt;")
            .replace('>', "&gt;")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_element(tag: &str) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node::create_element(tag)))
    }

    fn make_text(text: &str) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node::create_text(text)))
    }

    #[test]
    fn test_new() {
        let _s = HtmlSerializer::new();
    }

    #[test]
    fn test_inner_html_empty() {
        let node = make_element("div");
        assert_eq!(HtmlSerializer::inner_html(&node), "");
    }

    #[test]
    fn test_inner_html_with_text() {
        let parent = make_element("div");
        let child = make_text("hello");
        parent.borrow_mut().children.push(Rc::clone(&child));
        child.borrow_mut().parent = Some(Rc::clone(&parent));
        assert_eq!(HtmlSerializer::inner_html(&parent), "hello");
    }

    #[test]
    fn test_inner_html_nested() {
        let parent = make_element("div");
        let child = make_element("span");
        let text = make_text("word");
        child.borrow_mut().children.push(Rc::clone(&text));
        text.borrow_mut().parent = Some(Rc::clone(&child));
        parent.borrow_mut().children.push(Rc::clone(&child));
        child.borrow_mut().parent = Some(Rc::clone(&parent));
        assert_eq!(HtmlSerializer::inner_html(&parent), "<span>word</span>");
    }

    #[test]
    fn test_outer_html_element() {
        let node = make_element("br");
        assert_eq!(HtmlSerializer::outer_html(&node), "<br>");
    }

    #[test]
    fn test_outer_html_with_attrs() {
        let node = make_element("div");
        node.borrow_mut().set_attribute("id", "main");
        node.borrow_mut().set_attribute("class", "box");
        let html = HtmlSerializer::outer_html(&node);
        assert!(html.starts_with("<div"));
        assert!(html.ends_with("</div>"));
        assert!(html.contains("id=\"main\""));
        assert!(html.contains("class=\"box\""));
    }

    #[test]
    fn test_outer_html_text() {
        let node = make_text("hello & <world>");
        let html = HtmlSerializer::outer_html(&node);
        assert_eq!(html, "hello &amp; &lt;world&gt;");
    }

    #[test]
    fn test_outer_html_comment() {
        let node = Node::create_comment("todo");
        let rc = Rc::new(RefCell::new(node));
        assert_eq!(HtmlSerializer::outer_html(&rc), "<!--todo-->");
    }

    #[test]
    fn test_set_inner_html() {
        let node = make_element("div");
        HtmlSerializer::set_inner_html(&node, "<p>new</p>");
        assert_eq!(node.borrow().children.len(), 1);
        assert_eq!(
            node.borrow().children[0]
                .borrow()
                .node_value
                .clone()
                .unwrap(),
            "<p>new</p>"
        );
    }

    #[test]
    fn test_text_content() {
        let root = make_element("div");
        let t1 = make_text("hello ");
        let span = make_element("span");
        let t2 = make_text("world");
        span.borrow_mut().children.push(Rc::clone(&t2));
        t2.borrow_mut().parent = Some(Rc::clone(&span));
        root.borrow_mut().children.push(Rc::clone(&t1));
        t1.borrow_mut().parent = Some(Rc::clone(&root));
        root.borrow_mut().children.push(Rc::clone(&span));
        span.borrow_mut().parent = Some(Rc::clone(&root));

        assert_eq!(HtmlSerializer::text_content(&root), "hello world");
    }

    #[test]
    fn test_escape_text_escaping() {
        let node = make_text("a&b<c>d");
        let html = HtmlSerializer::outer_html(&node);
        assert_eq!(html, "a&amp;b&lt;c&gt;d");
    }

    #[test]
    fn test_escape_attr_escaping() {
        let node = make_element("div");
        node.borrow_mut().set_attribute("title", "a\"b<c>");
        let html = HtmlSerializer::outer_html(&node);
        assert!(html.contains("title=\"a&quot;b&lt;c&gt;\""));
    }

    #[test]
    fn test_void_element_no_closing() {
        let node = make_element("img");
        node.borrow_mut().set_attribute("src", "pic.png");
        let html = HtmlSerializer::outer_html(&node);
        assert!(html.starts_with("<img"));
        assert!(!html.contains("</img>"));
    }

    #[test]
    fn test_inner_html_multiple_children() {
        let parent = make_element("ul");
        let li1 = make_element("li");
        let t1 = make_text("one");
        li1.borrow_mut().children.push(Rc::clone(&t1));
        t1.borrow_mut().parent = Some(Rc::clone(&li1));
        let li2 = make_element("li");
        let t2 = make_text("two");
        li2.borrow_mut().children.push(Rc::clone(&t2));
        t2.borrow_mut().parent = Some(Rc::clone(&li2));
        parent.borrow_mut().children.push(Rc::clone(&li1));
        li1.borrow_mut().parent = Some(Rc::clone(&parent));
        parent.borrow_mut().children.push(Rc::clone(&li2));
        li2.borrow_mut().parent = Some(Rc::clone(&parent));

        let inner = HtmlSerializer::inner_html(&parent);
        assert_eq!(inner, "<li>one</li><li>two</li>");
    }

    #[test]
    fn test_text_content_on_text_node() {
        let node = make_text("only text");
        assert_eq!(HtmlSerializer::text_content(&node), "only text");
    }
}
