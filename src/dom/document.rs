use super::node::Node;

pub struct Document {
    pub root: Node,
}

impl Document {
    pub fn new() -> Self {
        Self {
            root: Node::new_element("html"),
        }
    }
}
