use crate::css::value::Value;
use std::collections::HashMap;

#[derive(Debug, Clone, Default)]
pub struct ComputedStyle {
    pub properties: HashMap<String, Value>,
}

impl ComputedStyle {
    pub fn new() -> Self {
        ComputedStyle {
            properties: HashMap::new(),
        }
    }

    pub fn get(&self, name: &str) -> Option<&Value> {
        self.properties.get(name)
    }

    pub fn set(&mut self, name: &str, value: Value) {
        self.properties.insert(name.to_string(), value);
    }

    pub fn merge(&mut self, other: &ComputedStyle) {
        for (k, v) in &other.properties {
            self.properties.insert(k.clone(), v.clone());
        }
    }

    pub fn inherit_from(&mut self, parent: &ComputedStyle) {
        let inherit_properties = vec![
            "color",
            "font-family",
            "font-size",
            "font-weight",
            "line-height",
        ];
        for prop in inherit_properties {
            if let Some(val) = parent.get(prop)
                && !self.properties.contains_key(prop) {
                    self.properties.insert(prop.to_string(), val.clone());
                }
        }
    }
}
