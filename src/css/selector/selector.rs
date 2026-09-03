#[derive(Debug, Clone, PartialEq)]
pub enum Selector {
    Universal,
    Type(String),
    Class(String),
    Id(String),
    Attribute {
        name: String,
        value: Option<String>,
        operator: Option<String>,
    },
    PseudoClass(String),
    PseudoElement(String),
    Descendant(Box<Selector>, Box<Selector>),
    Child(Box<Selector>, Box<Selector>),
    Adjacent(Box<Selector>, Box<Selector>),
    Sibling(Box<Selector>, Box<Selector>),
    List(Vec<Selector>),
}

impl Selector {
    pub fn is_empty(&self) -> bool {
        false
    }

    pub fn specificity(&self) -> (u32, u32, u32) {
        match self {
            Selector::Universal => (0, 0, 0),
            Selector::Type(_) => (0, 0, 1),
            Selector::Class(_) => (0, 1, 0),
            Selector::Id(_) => (1, 0, 0),
            Selector::Attribute { .. } => (0, 1, 0),
            Selector::PseudoClass(_) => (0, 1, 0),
            Selector::PseudoElement(_) => (0, 0, 1),
            Selector::Descendant(a, b)
            | Selector::Child(a, b)
            | Selector::Adjacent(a, b)
            | Selector::Sibling(a, b) => {
                let (a1, b1, c1) = a.specificity();
                let (a2, b2, c2) = b.specificity();
                (a1 + a2, b1 + b2, c1 + c2)
            }
            Selector::List(list) => {
                let mut max = (0, 0, 0);
                for sel in list {
                    let spec = sel.specificity();
                    if spec > max {
                        max = spec;
                    }
                }
                max
            }
        }
    }
}
