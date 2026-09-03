use crate::css::value::Value;

#[derive(Debug, Clone, PartialEq)]
pub struct Declaration {
    pub name: String,
    pub value: Value,
    pub important: bool,
}

impl Declaration {
    pub fn new(name: &str, value: Value) -> Self {
        Declaration {
            name: name.to_string(),
            value,
            important: false,
        }
    }

    pub fn with_importance(mut self, important: bool) -> Self {
        self.important = important;
        self
    }
}
