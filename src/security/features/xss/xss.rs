pub const HTML_ENTITIES: [(&str, &str); 5] = [
    ("&", "&amp;"),
    ("<", "&lt;"),
    (">", "&gt;"),
    ("\"", "&quot;"),
    ("'", "&#39;"),
];

pub struct XssFilter {
    pub encode_entities: bool,
    pub strip_scripts: bool,
    pub strip_event_handlers: bool,
    pub allowed_tags: Vec<String>,
}

impl XssFilter {
    pub fn new() -> Self {
        XssFilter {
            encode_entities: true,
            strip_scripts: true,
            strip_event_handlers: true,
            allowed_tags: Vec::new(),
        }
    }

    pub fn sanitize(&self, input: &str) -> String {
        let mut result = input.to_string();
        if self.strip_scripts {
            result = self.strip_script_tags(&result);
        }
        if self.strip_event_handlers {
            result = self.strip_event_handlers(&result);
        }
        if self.encode_entities {
            result = self.encode(&result);
        }
        result
    }

    pub fn encode(&self, input: &str) -> String {
        let mut out = String::with_capacity(input.len());
        for c in input.chars() {
            match c {
                '&' => out.push_str("&amp;"),
                '<' => out.push_str("&lt;"),
                '>' => out.push_str("&gt;"),
                '"' => out.push_str("&quot;"),
                '\'' => out.push_str("&#39;"),
                _ => out.push(c),
            }
        }
        out
    }

    pub fn strip_script_tags(&self, input: &str) -> String {
        let mut result = input.to_string();
        loop {
            let lower = result.to_lowercase();
            let start = lower.find("<script").and_then(|i| {
                let end = lower[i..].find('>').map(|j| i + j + 1);
                end
            });
            let Some(start) = start else { break };
            let close = result[start..].to_lowercase().find("</script>");
            let Some(close) = close else {
                result.replace_range(.., "");
                break;
            };
            let after = start + close + "</script>".len();
            result.replace_range(..after, "");
        }
        result
    }

    pub fn strip_event_handlers(&self, input: &str) -> String {
        let mut result = String::new();
        let rest = input;
        let mut idx = 0;
        while idx < rest.len() {
            let lower = rest.to_lowercase();
            let marker = lower[idx..].find("on");
            let Some(marker_pos) = marker else {
                result.push_str(&rest[idx..]);
                break;
            };
            let abs = idx + marker_pos;
            let end_quote = rest[abs..].find(['"', '\'']);
            result.push_str(&rest[idx..abs]);
            match end_quote {
                Some(q) => {
                    let quote = rest.as_bytes()[abs + q];
                    let close = rest[abs + q + 1..].find(quote as char);
                    let consumed = match close {
                        Some(c) => abs + q + 1 + c + 1,
                        None => rest.len(),
                    };
                    idx = consumed;
                }
                None => {
                    idx = abs + 2;
                }
            }
        }
        result
    }

    pub fn has_script_content(&self, input: &str) -> bool {
        let lower = input.to_lowercase();
        lower.contains("<script") || lower.contains("javascript:")
    }

    pub fn allow_tag(&mut self, tag: &str) {
        if !self.allowed_tags.contains(&tag.to_string()) {
            self.allowed_tags.push(tag.to_string());
        }
    }

    pub fn is_tag_allowed(&self, tag: &str) -> bool {
        self.allowed_tags.is_empty() || self.allowed_tags.iter().any(|t| t == tag)
    }
}

impl Default for XssFilter {
    fn default() -> Self {
        XssFilter::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encodes_html_characters() {
        let filter = XssFilter::new();
        assert_eq!(filter.encode("<script>"), "&lt;script&gt;");
        assert_eq!(filter.encode("a&b"), "a&amp;b");
    }

    #[test]
    fn strips_script_blocks() {
        let filter = XssFilter::new();
        let input = "<div>ok</div><script>alert(1)</script><p>hi</p>";
        let output = filter.strip_script_tags(input);
        assert!(!output.contains("<script"));
        assert!(!output.contains("alert(1)"));
    }

    #[test]
    fn strips_event_handlers() {
        let filter = XssFilter::new();
        let input = "<img src=x onerror=alert(1)>";
        let output = filter.strip_event_handlers(input);
        assert!(!output.contains("onerror"));
    }

    #[test]
    fn full_sanitize_removes_malicious_content() {
        let filter = XssFilter::new();
        let input = "<script>alert('x')</script><a href=\"javascript:evil()\">click</a>";
        let output = filter.sanitize(input);
        assert!(!output.contains("<script"));
        assert!(!output.contains("javascript:"));
    }

    #[test]
    fn detects_script_content() {
        let filter = XssFilter::new();
        assert!(filter.has_script_content("<script>"));
        assert!(filter.has_script_content("javascript:void(0)"));
        assert!(!filter.has_script_content("plain text"));
    }

    #[test]
    fn tag_allowlist() {
        let mut filter = XssFilter::new();
        filter.allow_tag("b");
        assert!(filter.is_tag_allowed("b"));
        assert!(!filter.is_tag_allowed("script"));
    }
}
