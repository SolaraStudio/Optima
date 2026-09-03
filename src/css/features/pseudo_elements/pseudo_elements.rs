use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PseudoElement {
    Before,
    After,
    FirstLine,
    FirstLetter,
    Marker,
    Placeholder,
    Selection,
    Backdrop,
}

#[derive(Debug, Clone)]
pub struct PseudoElementStyle {
    pub content: Option<String>,
    pub display: Option<String>,
    pub position: Option<String>,
    pub properties: HashMap<String, String>,
}

impl PseudoElementStyle {
    pub fn new() -> Self {
        PseudoElementStyle {
            content: None,
            display: None,
            position: None,
            properties: HashMap::new(),
        }
    }

    pub fn set_content(&mut self, content: &str) {
        self.content = Some(content.to_string());
    }

    pub fn set_property(&mut self, name: &str, value: &str) {
        match name {
            "display" => self.display = Some(value.to_string()),
            "position" => self.position = Some(value.to_string()),
            _ => {
                self.properties.insert(name.to_string(), value.to_string());
            }
        }
    }

    pub fn get_property(&self, name: &str) -> Option<&str> {
        match name {
            "display" => self.display.as_deref(),
            "position" => self.position.as_deref(),
            "content" => self.content.as_deref(),
            _ => self.properties.get(name).map(|s| s.as_str()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct PseudoElementGenerator {
    styles: HashMap<PseudoElement, PseudoElementStyle>,
}

impl PseudoElementGenerator {
    pub fn new() -> Self {
        PseudoElementGenerator {
            styles: HashMap::new(),
        }
    }

    pub fn set_style(&mut self, element: PseudoElement, style: PseudoElementStyle) {
        self.styles.insert(element, style);
    }

    pub fn get_style(&self, element: &PseudoElement) -> Option<&PseudoElementStyle> {
        self.styles.get(element)
    }

    pub fn has_element(&self, element: &PseudoElement) -> bool {
        self.styles.contains_key(element)
    }

    pub fn generate_before_after(&self, tag: &str) -> Vec<GeneratedBox> {
        let mut boxes = Vec::new();
        if let Some(style) = self.styles.get(&PseudoElement::Before) {
            let content = style.content.clone().unwrap_or_default();
            let resolved = resolve_content(&content, tag);
            boxes.push(GeneratedBox {
                pseudo_element: PseudoElement::Before,
                content: resolved,
                display: style
                    .display
                    .clone()
                    .unwrap_or_else(|| "inline".to_string()),
            });
        }
        if let Some(style) = self.styles.get(&PseudoElement::After) {
            let content = style.content.clone().unwrap_or_default();
            let resolved = resolve_content(&content, tag);
            boxes.push(GeneratedBox {
                pseudo_element: PseudoElement::After,
                content: resolved,
                display: style
                    .display
                    .clone()
                    .unwrap_or_else(|| "inline".to_string()),
            });
        }
        boxes
    }

    pub fn elements(&self) -> Vec<&PseudoElement> {
        self.styles.keys().collect()
    }
}

#[derive(Debug, Clone)]
pub struct GeneratedBox {
    pub pseudo_element: PseudoElement,
    pub content: String,
    pub display: String,
}

fn resolve_content(content: &str, _tag: &str) -> String {
    if content == "attr(data-text)" || content == "attr(data-text)" {
        return String::new();
    }
    if content.starts_with("\"") && content.ends_with("\"") {
        return content[1..content.len() - 1].to_string();
    }
    if content == "counter(list)" {
        return "1".to_string();
    }
    content.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_before_after_generation() {
        let mut generator = PseudoElementGenerator::new();
        let mut before = PseudoElementStyle::new();
        before.set_content("\"Hello\"");
        before.set_property("display", "block");
        generator.set_style(PseudoElement::Before, before);
        let mut after = PseudoElementStyle::new();
        after.set_content("\"World\"");
        generator.set_style(PseudoElement::After, after);
        let boxes = generator.generate_before_after("div");
        assert_eq!(boxes.len(), 2);
        assert_eq!(boxes[0].content, "Hello");
        assert_eq!(boxes[0].display, "block");
        assert_eq!(boxes[1].content, "World");
    }

    #[test]
    fn test_pseudo_element_style_properties() {
        let mut style = PseudoElementStyle::new();
        style.set_property("color", "red");
        style.set_property("font-size", "14px");
        assert_eq!(style.get_property("color"), Some("red"));
        assert_eq!(style.get_property("font-size"), Some("14px"));
        assert_eq!(style.get_property("missing"), None);
    }

    #[test]
    fn test_display_and_position() {
        let mut style = PseudoElementStyle::new();
        style.set_property("display", "flex");
        style.set_property("position", "absolute");
        assert_eq!(style.display, Some("flex".to_string()));
        assert_eq!(style.position, Some("absolute".to_string()));
    }

    #[test]
    fn test_has_element() {
        let mut generator = PseudoElementGenerator::new();
        assert!(!generator.has_element(&PseudoElement::Before));
        generator.set_style(PseudoElement::Before, PseudoElementStyle::new());
        assert!(generator.has_element(&PseudoElement::Before));
    }

    #[test]
    fn test_elements_list() {
        let mut generator = PseudoElementGenerator::new();
        generator.set_style(PseudoElement::Before, PseudoElementStyle::new());
        generator.set_style(PseudoElement::After, PseudoElementStyle::new());
        let elements = generator.elements();
        assert_eq!(elements.len(), 2);
    }

    #[test]
    fn test_resolve_content_quotes() {
        assert_eq!(resolve_content("\"test\"", "div"), "test");
    }

    #[test]
    fn test_resolve_content_plain() {
        assert_eq!(resolve_content("none", "div"), "none");
    }

    #[test]
    fn test_get_style() {
        let mut generator = PseudoElementGenerator::new();
        generator.set_style(PseudoElement::FirstLine, PseudoElementStyle::new());
        assert!(generator.get_style(&PseudoElement::FirstLine).is_some());
        assert!(generator.get_style(&PseudoElement::Selection).is_none());
    }
}
