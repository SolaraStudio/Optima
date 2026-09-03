use std::collections::HashMap;

pub struct CssVariables {
    map: HashMap<String, String>,
    inherited: HashMap<String, String>,
}

impl Default for CssVariables {
    fn default() -> Self {
        Self::new()
    }
}

impl CssVariables {
    pub fn new() -> Self {
        CssVariables {
            map: HashMap::new(),
            inherited: HashMap::new(),
        }
    }

    pub fn set(&mut self, name: &str, value: &str) {
        if let Some(key) = name.strip_prefix("--") {
            self.map.insert(key.to_string(), value.to_string());
        }
    }

    pub fn set_inherited(&mut self, name: &str, value: &str) {
        if let Some(key) = name.strip_prefix("--") {
            self.inherited.insert(key.to_string(), value.to_string());
        }
    }

    pub fn get(&self, name: &str) -> Option<&str> {
        if let Some(key) = name.strip_prefix("--") {
            self.map
                .get(key)
                .or_else(|| self.inherited.get(key))
                .map(|s| s.as_str())
        } else {
            None
        }
    }

    pub fn resolve(&self, input: &str) -> String {
        let mut result = input.to_string();
        let mut iterations = 0;
        while result.contains("var(") && iterations < 16 {
            let mut new_result = String::with_capacity(result.len());
            let mut chars = result.char_indices().peekable();
            while let Some((i, ch)) = chars.next() {
                if i + 4 <= result.len() && &result[i..i + 4] == "var(" {
                    let rest = &result[i + 4..];
                    if let Some(paren_end) = rest.find(')') {
                        let var_expr = &rest[..paren_end];
                        let (var_name, fallback) = match var_expr.find(',') {
                            Some(comma_pos) => (
                                var_expr[..comma_pos].trim(),
                                Some(var_expr[comma_pos + 1..].trim()),
                            ),
                            None => (var_expr.trim(), None),
                        };
                        if let Some(val) = self.get(var_name) {
                            new_result.push_str(val);
                        } else if let Some(fb) = fallback {
                            new_result.push_str(fb);
                        }
                        let skip = paren_end + 1;
                        for _ in 0..skip {
                            chars.next();
                        }
                    } else {
                        new_result.push(ch);
                    }
                } else {
                    new_result.push(ch);
                }
            }
            result = new_result;
            iterations += 1;
        }
        result
    }

    pub fn keys(&self) -> Vec<String> {
        let mut all: Vec<String> = self.inherited.keys().cloned().collect();
        for k in self.map.keys() {
            if !all.contains(k) {
                all.push(k.clone());
            }
        }
        all
    }

    pub fn merge(&mut self, other: &CssVariables) {
        for (k, v) in &other.inherited {
            self.inherited.entry(k.clone()).or_insert_with(|| v.clone());
        }
        for (k, v) in &other.map {
            self.map.insert(k.clone(), v.clone());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_and_get() {
        let mut vars = CssVariables::new();
        vars.set("--color", "red");
        assert_eq!(vars.get("--color"), Some("red"));
    }

    #[test]
    fn test_get_without_prefix() {
        let vars = CssVariables::new();
        assert_eq!(vars.get("color"), None);
    }

    #[test]
    fn test_inherited_takes_lower_priority() {
        let mut vars = CssVariables::new();
        vars.set_inherited("--color", "blue");
        vars.set("--color", "red");
        assert_eq!(vars.get("--color"), Some("red"));
    }

    #[test]
    fn test_resolve_simple() {
        let mut vars = CssVariables::new();
        vars.set("--bg", "#fff");
        let resolved = vars.resolve("var(--bg)");
        assert_eq!(resolved, "#fff");
    }

    #[test]
    fn test_resolve_with_fallback() {
        let vars = CssVariables::new();
        let resolved = vars.resolve("var(--missing, 10px)");
        assert_eq!(resolved, "10px");
    }

    #[test]
    fn test_resolve_nested() {
        let mut vars = CssVariables::new();
        vars.set("--a", "var(--b)");
        vars.set("--b", "42px");
        let resolved = vars.resolve("var(--a)");
        assert_eq!(resolved, "42px");
    }

    #[test]
    fn test_keys() {
        let mut vars = CssVariables::new();
        vars.set("--x", "1");
        vars.set_inherited("--y", "2");
        let mut keys = vars.keys();
        keys.sort();
        assert_eq!(keys, vec!["x", "y"]);
    }

    #[test]
    fn test_merge() {
        let mut a = CssVariables::new();
        a.set("--x", "1");
        let mut b = CssVariables::new();
        b.set("--y", "2");
        b.set("--x", "override");
        a.merge(&b);
        assert_eq!(a.get("--x"), Some("override"));
        assert_eq!(a.get("--y"), Some("2"));
    }
}
