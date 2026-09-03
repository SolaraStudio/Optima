use crate::css::declaration::Declaration;
use crate::css::selector::Selector;
use crate::css::specificity::Specificity;
use crate::css::stylesheet::Stylesheet;
use std::collections::HashMap;

pub struct Cascade;

impl Cascade {
    pub fn resolve(stylesheet: &Stylesheet, element: &Element) -> HashMap<String, Declaration> {
        let mut declarations = HashMap::new();
        for rule in &stylesheet.rules {
            for selector in &rule.selectors {
                if Cascade::matches(selector, element) {
                    let specificity = Specificity::from_selector(selector);
                    for decl in &rule.declarations {
                        let key = decl.name.clone();
                        let entry = declarations.entry(key).or_insert_with(|| decl.clone());
                        if specificity > Specificity::from_selector(selector) {
                            *entry = decl.clone();
                        }
                    }
                }
            }
        }
        declarations
    }

    pub fn matches(selector: &Selector, element: &Element) -> bool {
        match selector {
            Selector::Universal => true,
            Selector::Type(tag) => element.tag == *tag,
            Selector::Class(class) => element.classes.contains(class),
            Selector::Id(id) => element.id == *id,
            Selector::Attribute {
                name,
                value,
                operator,
            } => {
                if let Some(val) = element.attributes.get(name) {
                    match operator {
                        Some(op) if op == "=" => val == value.as_ref().unwrap_or(&"".to_string()),
                        Some(op) if op == "~=" => val
                            .split_whitespace()
                            .any(|v| v == value.as_ref().unwrap_or(&"".to_string())),
                        Some(op) if op == "|=" => val
                            .split('-')
                            .any(|v| v == value.as_ref().unwrap_or(&"".to_string())),
                        Some(op) if op == "^=" => {
                            val.starts_with(value.as_ref().unwrap_or(&"".to_string()))
                        }
                        Some(op) if op == "$=" => {
                            val.ends_with(value.as_ref().unwrap_or(&"".to_string()))
                        }
                        Some(op) if op == "*=" => {
                            val.contains(value.as_ref().unwrap_or(&"".to_string()))
                        }
                        _ => true,
                    }
                } else {
                    false
                }
            }
            Selector::PseudoClass(name) => match name.as_str() {
                "first-child" => element.is_first_child,
                "last-child" => element.is_last_child,
                "only-child" => element.is_only_child,
                "empty" => element.is_empty,
                _ => false,
            },
            Selector::PseudoElement(_) => true,
            Selector::Descendant(a, b) => {
                Cascade::matches(a, element) && Cascade::matches(b, element)
            }
            Selector::Child(a, b) => Cascade::matches(a, element) && Cascade::matches(b, element),
            Selector::Adjacent(a, b) => {
                Cascade::matches(a, element) && Cascade::matches(b, element)
            }
            Selector::Sibling(a, b) => Cascade::matches(a, element) && Cascade::matches(b, element),
            Selector::List(list) => list.iter().any(|sel| Cascade::matches(sel, element)),
        }
    }
}

pub struct Element {
    pub tag: String,
    pub id: String,
    pub classes: Vec<String>,
    pub attributes: HashMap<String, String>,
    pub is_first_child: bool,
    pub is_last_child: bool,
    pub is_only_child: bool,
    pub is_empty: bool,
}
