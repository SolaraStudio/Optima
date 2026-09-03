use crate::css::declaration::Declaration;
use crate::css::selector::Selector;
use crate::css::value::Value;

pub struct Stylesheet {
    pub rules: Vec<Rule>,
}

pub struct Rule {
    pub selectors: Vec<Selector>,
    pub declarations: Vec<Declaration>,
}

pub struct AtRule {
    pub name: String,
    pub prelude: Option<String>,
    pub block: Vec<Rule>,
}

pub struct DeclarationBlock {
    pub declarations: Vec<Declaration>,
}

pub struct SelectorList {
    pub selectors: Vec<Selector>,
}

pub struct ValueList {
    pub values: Vec<Value>,
}
