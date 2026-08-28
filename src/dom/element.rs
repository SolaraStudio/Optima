//! DOM Element implementation
pub struct Element {
    pub tag: String,
    pub attributes: std::collections::HashMap<String, String>,
}
impl Element { pub fn new(tag: &str) -> Self { Self { tag: tag.to_string(), attributes: std::collections::HashMap::new() } } }
