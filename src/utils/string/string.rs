pub fn trim_whitespace(s: &str) -> String {
    s.trim().to_string()
}

pub fn trim_newlines(s: &str) -> String {
    s.trim_end_matches('\n').trim_end_matches('\r').to_string()
}

pub fn split_lines(s: &str) -> Vec<&str> {
    s.lines().collect()
}

pub fn split_words(s: &str) -> Vec<&str> {
    s.split_whitespace().collect()
}

pub fn to_lowercase(s: &str) -> String {
    s.to_lowercase()
}

pub fn to_uppercase(s: &str) -> String {
    s.to_uppercase()
}

pub fn capitalize_first(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().chain(chars).collect(),
    }
}

pub fn truncate(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else {
        format!("{}...", &s[..max_len.saturating_sub(3)])
    }
}

pub fn contains_any(s: &str, patterns: &[&str]) -> bool {
    patterns.iter().any(|p| s.contains(*p))
}

pub fn starts_with_any(s: &str, patterns: &[&str]) -> bool {
    patterns.iter().any(|p| s.starts_with(*p))
}

pub fn ends_with_any(s: &str, patterns: &[&str]) -> bool {
    patterns.iter().any(|p| s.ends_with(*p))
}

pub fn remove_prefix(s: &str, prefix: &str) -> String {
    if let Some(rest) = s.strip_prefix(prefix) {
        rest.to_string()
    } else {
        s.to_string()
    }
}

pub fn remove_suffix(s: &str, suffix: &str) -> String {
    if let Some(rest) = s.strip_suffix(suffix) {
        rest.to_string()
    } else {
        s.to_string()
    }
}

pub fn replace_all(s: &str, from: &str, to: &str) -> String {
    s.replace(from, to)
}

pub fn is_empty_or_whitespace(s: &str) -> bool {
    s.trim().is_empty()
}

pub fn count_occurrences(s: &str, pattern: &str) -> usize {
    s.matches(pattern).count()
}

pub fn reverse(s: &str) -> String {
    s.chars().rev().collect()
}
