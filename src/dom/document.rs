//! DOM Document implementation
use super::{Node, Element};
pub struct Document {
    pub root: Node,
}
impl Document { pub fn new() -> Self { Self { root: Node::new() } } }
