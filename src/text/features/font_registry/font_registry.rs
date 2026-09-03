use crate::text::font::{FontFace, FontMetrics, FontStyle, FontWeight};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct FontDescriptor {
    pub family: String,
    pub weight: FontWeight,
    pub style: FontStyle,
    pub source_url: Option<String>,
    pub loaded: bool,
}

pub struct WebFontRegistry {
    fonts: HashMap<String, Vec<FontFace>>,
    descriptors: Vec<FontDescriptor>,
    aliases: HashMap<String, String>,
}

impl WebFontRegistry {
    pub fn new() -> Self {
        WebFontRegistry {
            fonts: HashMap::new(),
            descriptors: Vec::new(),
            aliases: HashMap::new(),
        }
    }

    pub fn register(&mut self, face: FontFace) {
        self.descriptors.push(FontDescriptor {
            family: face.family.clone(),
            weight: face.weight,
            style: face.style,
            source_url: None,
            loaded: true,
        });
        self.fonts
            .entry(face.family.clone())
            .or_insert_with(Vec::new)
            .push(face);
    }

    pub fn register_descriptor(&mut self, descriptor: FontDescriptor) {
        self.descriptors.push(descriptor);
    }

    pub fn unregister(&mut self, family: &str) -> bool {
        let had = self.fonts.remove(family).is_some();
        self.descriptors.retain(|d| d.family != family);
        self.aliases.retain(|_, v| v != family);
        had
    }

    pub fn lookup(
        &self,
        family: &str,
        weight: FontWeight,
        style: FontStyle,
    ) -> Option<&FontFace> {
        let resolved = self.aliases.get(family).map(|s| s.as_str()).unwrap_or(family);
        self.fonts.get(resolved)?.iter().find(|f| {
            f.weight == weight && f.style == style
        }).or_else(|| {
            self.fonts.get(resolved)?.iter().find(|f| {
                f.style == style
            })
        }).or_else(|| {
            self.fonts.get(resolved)?.first()
        })
    }

    pub fn lookup_best_match(
        &self,
        family: &str,
        weight: FontWeight,
        style: FontStyle,
    ) -> Option<&FontFace> {
        self.lookup(family, weight, style)
    }

    pub fn has_family(&self, family: &str) -> bool {
        let resolved = self.aliases.get(family).map(|s| s.as_str()).unwrap_or(family);
        self.fonts.contains_key(resolved)
    }

    pub fn family_count(&self) -> usize {
        self.fonts.len()
    }

    pub fn font_count(&self) -> usize {
        self.fonts.values().map(|v| v.len()).sum()
    }

    pub fn families(&self) -> Vec<&str> {
        self.fonts.keys().map(|s| s.as_str()).collect()
    }

    pub fn add_alias(&mut self, alias: &str, target: &str) {
        self.aliases.insert(alias.to_string(), target.to_string());
    }

    pub fn resolve_alias<'a>(&'a self, family: &'a str) -> &'a str {
        self.aliases
            .get(family)
            .map(|s| s.as_str())
            .unwrap_or(family)
    }

    pub fn descriptors_for_family(&self, family: &str) -> Vec<&FontDescriptor> {
        self.descriptors
            .iter()
            .filter(|d| d.family == family)
            .collect()
    }

    pub fn get_metrics(&self, family: &str) -> Option<&FontMetrics> {
        let resolved = self.aliases.get(family).map(|s| s.as_str()).unwrap_or(family);
        self.fonts
            .get(resolved)
            .and_then(|faces| faces.first())
            .map(|f| &f.metrics)
    }

    pub fn clear(&mut self) {
        self.fonts.clear();
        self.descriptors.clear();
        self.aliases.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_face(family: &str, weight: FontWeight, style: FontStyle) -> FontFace {
        FontFace {
            family: family.to_string(),
            weight,
            style,
            data: vec![],
            metrics: FontMetrics::default(),
        }
    }

    #[test]
    fn test_new() {
        let reg = WebFontRegistry::new();
        assert_eq!(reg.family_count(), 0);
        assert_eq!(reg.font_count(), 0);
    }

    #[test]
    fn test_register() {
        let mut reg = WebFontRegistry::new();
        reg.register(make_face("Roboto", FontWeight::Regular, FontStyle::Normal));
        assert!(reg.has_family("Roboto"));
        assert_eq!(reg.family_count(), 1);
        assert_eq!(reg.font_count(), 1);
    }

    #[test]
    fn test_register_multiple_faces() {
        let mut reg = WebFontRegistry::new();
        reg.register(make_face("Roboto", FontWeight::Regular, FontStyle::Normal));
        reg.register(make_face("Roboto", FontWeight::Bold, FontStyle::Normal));
        reg.register(make_face("Roboto", FontWeight::Regular, FontStyle::Italic));
        assert_eq!(reg.font_count(), 3);
    }

    #[test]
    fn test_unregister() {
        let mut reg = WebFontRegistry::new();
        reg.register(make_face("Test", FontWeight::Regular, FontStyle::Normal));
        assert!(reg.unregister("Test"));
        assert!(!reg.has_family("Test"));
        assert!(!reg.unregister("Test"));
    }

    #[test]
    fn test_lookup_exact() {
        let mut reg = WebFontRegistry::new();
        reg.register(make_face("F1", FontWeight::Bold, FontStyle::Italic));
        let found = reg.lookup("F1", FontWeight::Bold, FontStyle::Italic);
        assert!(found.is_some());
        assert_eq!(found.unwrap().weight, FontWeight::Bold);
    }

    #[test]
    fn test_lookup_fallback_to_any_style() {
        let mut reg = WebFontRegistry::new();
        reg.register(make_face("F2", FontWeight::Regular, FontStyle::Normal));
        let found = reg.lookup("F2", FontWeight::Regular, FontStyle::Italic);
        assert!(found.is_some());
    }

    #[test]
    fn test_lookup_fallback_to_first() {
        let mut reg = WebFontRegistry::new();
        reg.register(make_face("F3", FontWeight::Bold, FontStyle::Normal));
        let found = reg.lookup("F3", FontWeight::Black, FontStyle::Oblique);
        assert!(found.is_some());
    }

    #[test]
    fn test_lookup_not_found() {
        let reg = WebFontRegistry::new();
        assert!(reg.lookup("Missing", FontWeight::Regular, FontStyle::Normal).is_none());
    }

    #[test]
    fn test_alias() {
        let mut reg = WebFontRegistry::new();
        reg.register(make_face("Open Sans", FontWeight::Regular, FontStyle::Normal));
        reg.add_alias("sans", "Open Sans");
        assert!(reg.has_family("sans"));
        assert_eq!(reg.resolve_alias("sans"), "Open Sans");
    }

    #[test]
    fn test_resolve_no_alias() {
        let reg = WebFontRegistry::new();
        assert_eq!(reg.resolve_alias("whatever"), "whatever");
    }

    #[test]
    fn test_families() {
        let mut reg = WebFontRegistry::new();
        reg.register(make_face("A", FontWeight::Regular, FontStyle::Normal));
        reg.register(make_face("B", FontWeight::Regular, FontStyle::Normal));
        let mut families = reg.families();
        families.sort();
        assert_eq!(families, vec!["A", "B"]);
    }

    #[test]
    fn test_descriptors_for_family() {
        let mut reg = WebFontRegistry::new();
        reg.register(make_face("X", FontWeight::Regular, FontStyle::Normal));
        reg.register(make_face("X", FontWeight::Bold, FontStyle::Normal));
        reg.register(make_face("Y", FontWeight::Regular, FontStyle::Normal));
        let descs = reg.descriptors_for_family("X");
        assert_eq!(descs.len(), 2);
    }

    #[test]
    fn test_clear() {
        let mut reg = WebFontRegistry::new();
        reg.register(make_face("A", FontWeight::Regular, FontStyle::Normal));
        reg.clear();
        assert_eq!(reg.family_count(), 0);
        assert!(!reg.has_family("A"));
    }

    #[test]
    fn test_unregister_removes_alias() {
        let mut reg = WebFontRegistry::new();
        reg.register(make_face("Real", FontWeight::Regular, FontStyle::Normal));
        reg.add_alias("alias", "Real");
        reg.unregister("Real");
        assert!(!reg.has_family("alias"));
    }
}
