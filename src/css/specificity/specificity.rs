#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Specificity {
    pub a: u32,
    pub b: u32,
    pub c: u32,
}

impl Specificity {
    pub fn new(a: u32, b: u32, c: u32) -> Self {
        Specificity { a, b, c }
    }

    pub fn from_selector(selector: &crate::css::selector::Selector) -> Self {
        let (a, b, c) = selector.specificity();
        Specificity { a, b, c }
    }
}
