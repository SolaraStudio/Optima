#[derive(Debug, Clone, PartialEq)]
pub enum PseudoClass {
    Hover,
    Active,
    Focus,
    FocusVisible,
    FocusWithin,
    FirstChild,
    LastChild,
    NthChild(NthExpression),
    NthLastChild(NthExpression),
    NthOfType(NthExpression),
    NthLastOfType(NthExpression),
    FirstOfType,
    LastOfType,
    Empty,
    Root,
    Disabled,
    Enabled,
    Checked,
    ReadOnly,
    ReadWrite,
    Required,
    Optional,
    Valid,
    Invalid,
    Link,
    Visited,
    Lang(String),
    Dir(Direction),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Direction {
    Ltr,
    Rtl,
}

#[derive(Debug, Clone, PartialEq)]
pub struct NthExpression {
    pub a: i32,
    pub b: i32,
}

impl NthExpression {
    pub fn new(a: i32, b: i32) -> Self {
        NthExpression { a, b }
    }

    pub fn matches(&self, index: usize) -> bool {
        let n = index as i32;
        if self.a == 0 {
            return n == self.b - 1;
        }
        let mut i = 0;
        loop {
            let result = self.a * i + self.b;
            if result == n as i32 + 1 {
                return true;
            }
            if result > n + 1 && self.a > 0 {
                return false;
            }
            if result < n + 1 && self.a < 0 {
                return false;
            }
            i += if self.a > 0 { 1 } else { -1 };
            if i.abs() > 1000 {
                return false;
            }
        }
    }

    pub fn from_odd() -> Self {
        NthExpression { a: 2, b: 1 }
    }

    pub fn from_even() -> Self {
        NthExpression { a: 2, b: 0 }
    }
}

#[derive(Debug, Clone)]
pub struct ElementState {
    pub is_hovered: bool,
    pub is_active: bool,
    pub is_focused: bool,
    pub focus_visible: bool,
    pub child_index: usize,
    pub sibling_count: usize,
    pub type_index: usize,
    pub type_sibling_count: usize,
    pub is_first_child: bool,
    pub is_last_child: bool,
    pub is_empty: bool,
    pub is_root: bool,
    pub is_disabled: bool,
    pub is_enabled: bool,
    pub is_checked: bool,
    pub is_read_only: bool,
    pub is_required: bool,
    pub is_valid: bool,
    pub is_visited: bool,
    pub lang: String,
    pub direction: Direction,
}

impl ElementState {
    pub fn new() -> Self {
        ElementState {
            is_hovered: false,
            is_active: false,
            is_focused: false,
            focus_visible: false,
            child_index: 0,
            sibling_count: 0,
            type_index: 0,
            type_sibling_count: 0,
            is_first_child: false,
            is_last_child: false,
            is_empty: true,
            is_root: false,
            is_disabled: false,
            is_enabled: true,
            is_checked: false,
            is_read_only: false,
            is_required: false,
            is_valid: true,
            is_visited: false,
            lang: String::new(),
            direction: Direction::Ltr,
        }
    }
}

impl PseudoClass {
    pub fn matches(&self, state: &ElementState) -> bool {
        match self {
            PseudoClass::Hover => state.is_hovered,
            PseudoClass::Active => state.is_active,
            PseudoClass::Focus => state.is_focused,
            PseudoClass::FocusVisible => state.focus_visible,
            PseudoClass::FocusWithin => state.is_focused,
            PseudoClass::FirstChild => state.is_first_child,
            PseudoClass::LastChild => state.is_last_child,
            PseudoClass::NthChild(expr) => expr.matches(state.child_index),
            PseudoClass::NthLastChild(expr) => {
                let from_end = state.sibling_count - state.child_index;
                expr.matches(from_end)
            }
            PseudoClass::NthOfType(expr) => expr.matches(state.type_index),
            PseudoClass::NthLastOfType(expr) => {
                let from_end = state.type_sibling_count - state.type_index;
                expr.matches(from_end)
            }
            PseudoClass::FirstOfType => state.type_index == 0,
            PseudoClass::LastOfType => {
                state.type_index + 1 == state.type_sibling_count
            }
            PseudoClass::Empty => state.is_empty,
            PseudoClass::Root => state.is_root,
            PseudoClass::Disabled => state.is_disabled,
            PseudoClass::Enabled => state.is_enabled,
            PseudoClass::Checked => state.is_checked,
            PseudoClass::ReadOnly => state.is_read_only,
            PseudoClass::ReadWrite => !state.is_read_only,
            PseudoClass::Required => state.is_required,
            PseudoClass::Optional => !state.is_required,
            PseudoClass::Valid => state.is_valid,
            PseudoClass::Invalid => !state.is_valid,
            PseudoClass::Link => !state.is_visited,
            PseudoClass::Visited => state.is_visited,
            PseudoClass::Lang(lang) => state.lang == *lang,
            PseudoClass::Dir(dir) => state.direction == *dir,
        }
    }
}

pub fn match_pseudo_classes(classes: &[PseudoClass], state: &ElementState) -> bool {
    classes.iter().all(|pc| pc.matches(state))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hover() {
        let mut state = ElementState::new();
        assert!(!PseudoClass::Hover.matches(&state));
        state.is_hovered = true;
        assert!(PseudoClass::Hover.matches(&state));
    }

    #[test]
    fn test_active() {
        let mut state = ElementState::new();
        state.is_active = true;
        assert!(PseudoClass::Active.matches(&state));
    }

    #[test]
    fn test_focus() {
        let mut state = ElementState::new();
        state.is_focused = true;
        assert!(PseudoClass::Focus.matches(&state));
    }

    #[test]
    fn test_first_child() {
        let mut state = ElementState::new();
        state.is_first_child = true;
        assert!(PseudoClass::FirstChild.matches(&state));
    }

    #[test]
    fn test_last_child() {
        let mut state = ElementState::new();
        state.is_last_child = true;
        assert!(PseudoClass::LastChild.matches(&state));
    }

    #[test]
    fn test_nth_child_odd() {
        let expr = NthExpression::from_odd();
        assert!(expr.matches(0));
        assert!(!expr.matches(1));
        assert!(expr.matches(2));
    }

    #[test]
    fn test_nth_child_even() {
        let expr = NthExpression::from_even();
        assert!(!expr.matches(0));
        assert!(expr.matches(1));
        assert!(!expr.matches(2));
    }

    #[test]
    fn test_nth_child_2n_plus_1() {
        let expr = NthExpression::new(2, 1);
        assert!(expr.matches(0));
        assert!(!expr.matches(1));
        assert!(expr.matches(2));
    }

    #[test]
    fn test_nth_child_3n() {
        let expr = NthExpression::new(3, 0);
        assert!(!expr.matches(0));
        assert!(!expr.matches(1));
        assert!(!expr.matches(2));
        assert!(expr.matches(3));
        assert!(!expr.matches(4));
    }

    #[test]
    fn test_empty() {
        let state = ElementState::new();
        assert!(PseudoClass::Empty.matches(&state));
    }

    #[test]
    fn test_disabled() {
        let mut state = ElementState::new();
        state.is_disabled = true;
        state.is_enabled = false;
        assert!(PseudoClass::Disabled.matches(&state));
        assert!(!PseudoClass::Enabled.matches(&state));
    }

    #[test]
    fn test_checked() {
        let mut state = ElementState::new();
        state.is_checked = true;
        assert!(PseudoClass::Checked.matches(&state));
    }

    #[test]
    fn test_lang() {
        let mut state = ElementState::new();
        state.lang = "en".to_string();
        assert!(PseudoClass::Lang("en".to_string()).matches(&state));
        assert!(!PseudoClass::Lang("fr".to_string()).matches(&state));
    }

    #[test]
    fn test_match_pseudo_classes_all_pass() {
        let state = ElementState {
            is_hovered: true,
            is_active: true,
            ..ElementState::new()
        };
        let classes = vec![PseudoClass::Hover, PseudoClass::Active];
        assert!(match_pseudo_classes(&classes, &state));
    }

    #[test]
    fn test_match_pseudo_classes_one_fails() {
        let state = ElementState {
            is_hovered: true,
            is_active: false,
            ..ElementState::new()
        };
        let classes = vec![PseudoClass::Hover, PseudoClass::Active];
        assert!(!match_pseudo_classes(&classes, &state));
    }
}
