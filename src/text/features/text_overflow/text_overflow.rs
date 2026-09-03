#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OverflowMode {
    Clip,
    Ellipsis,
}

pub struct TextOverflowHandler {
    pub mode: OverflowMode,
    pub max_width: f32,
    pub ellipsis: String,
    pub char_width: f32,
}

impl TextOverflowHandler {
    pub fn new(mode: OverflowMode, max_width: f32, char_width: f32) -> Self {
        TextOverflowHandler {
            mode,
            max_width,
            ellipsis: "\u{2026}".to_string(),
            char_width,
        }
    }

    pub fn truncate(&self, text: &str) -> String {
        if text.is_empty() || self.max_width <= 0.0 {
            return String::new();
        }

        let char_count = text.chars().count();
        let full_width = char_count as f32 * self.char_width;

        if full_width <= self.max_width {
            return text.to_string();
        }

        match self.mode {
            OverflowMode::Clip => self.clip_text(text),
            OverflowMode::Ellipsis => self.ellipsis_text(text),
        }
    }

    fn clip_text(&self, text: &str) -> String {
        let mut result = String::new();
        let mut width = 0.0;
        for ch in text.chars() {
            if width + self.char_width > self.max_width {
                break;
            }
            result.push(ch);
            width += self.char_width;
        }
        result
    }

    fn ellipsis_text(&self, text: &str) -> String {
        let ellipsis_width = self.ellipsis.chars().count() as f32 * self.char_width;
        let available = self.max_width - ellipsis_width;
        if available <= 0.0 {
            return self.ellipsis.clone();
        }
        let mut result = String::new();
        let mut width = 0.0;
        for ch in text.chars() {
            if width + self.char_width > available {
                break;
            }
            result.push(ch);
            width += self.char_width;
        }
        result.push_str(&self.ellipsis);
        result
    }

    pub fn is_overflowing(&self, text: &str) -> bool {
        let char_count = text.chars().count();
        char_count as f32 * self.char_width > self.max_width
    }

    pub fn visible_char_count(&self, text: &str) -> usize {
        let mut count = 0usize;
        let mut width = 0.0;
        for _ in text.chars() {
            if width + self.char_width > self.max_width {
                break;
            }
            count += 1;
            width += self.char_width;
        }
        count
    }

    pub fn visible_width(&self, text: &str) -> f32 {
        let chars = self.visible_char_count(text);
        chars as f32 * self.char_width
    }

    pub fn overflow_width(&self, text: &str) -> f32 {
        let full = text.chars().count() as f32 * self.char_width;
        (full - self.max_width).max(0.0)
    }

    pub fn set_mode(&mut self, mode: OverflowMode) {
        self.mode = mode;
    }

    pub fn set_max_width(&mut self, width: f32) {
        self.max_width = width;
    }

    pub fn set_ellipsis(&mut self, ellipsis: &str) {
        self.ellipsis = ellipsis.to_string();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn handler(mode: OverflowMode, max_width: f32) -> TextOverflowHandler {
        TextOverflowHandler::new(mode, max_width, 10.0)
    }

    #[test]
    fn test_new() {
        let h = handler(OverflowMode::Clip, 100.0);
        assert_eq!(h.mode, OverflowMode::Clip);
        assert_eq!(h.max_width, 100.0);
    }

    #[test]
    fn test_clip_no_overflow() {
        let h = handler(OverflowMode::Clip, 100.0);
        assert_eq!(h.truncate("hello"), "hello");
    }

    #[test]
    fn test_clip_overflow() {
        let h = handler(OverflowMode::Clip, 30.0);
        assert_eq!(h.truncate("abcdefghij"), "abc");
    }

    #[test]
    fn test_clip_exact_fit() {
        let h = handler(OverflowMode::Clip, 50.0);
        assert_eq!(h.truncate("abcde"), "abcde");
    }

    #[test]
    fn test_ellipsis_no_overflow() {
        let h = handler(OverflowMode::Ellipsis, 100.0);
        assert_eq!(h.truncate("short"), "short");
    }

    #[test]
    fn test_ellipsis_overflow() {
        let h = handler(OverflowMode::Ellipsis, 40.0);
        let result = h.truncate("abcdefghijklmnop");
        assert!(result.ends_with('\u{2026}'));
        assert_eq!(result, "abc\u{2026}");
    }

    #[test]
    fn test_ellipsis_empty_text() {
        let h = handler(OverflowMode::Ellipsis, 50.0);
        assert_eq!(h.truncate(""), "");
    }

    #[test]
    fn test_ellipsis_very_narrow() {
        let h = handler(OverflowMode::Ellipsis, 5.0);
        let result = h.truncate("hi");
        assert_eq!(result, "\u{2026}");
    }

    #[test]
    fn test_is_overflowing() {
        let h = handler(OverflowMode::Clip, 50.0);
        assert!(!h.is_overflowing("abc"));
        assert!(h.is_overflowing("abcdefghij"));
    }

    #[test]
    fn test_visible_char_count() {
        let h = handler(OverflowMode::Clip, 30.0);
        assert_eq!(h.visible_char_count("abcdef"), 3);
        assert_eq!(h.visible_char_count("ab"), 2);
    }

    #[test]
    fn test_visible_width() {
        let h = handler(OverflowMode::Clip, 30.0);
        assert_eq!(h.visible_width("abcde"), 30.0);
        assert_eq!(h.visible_width("ab"), 20.0);
    }

    #[test]
    fn test_overflow_width() {
        let h = handler(OverflowMode::Clip, 30.0);
        assert_eq!(h.overflow_width("abc"), 0.0);
        assert_eq!(h.overflow_width("abcde"), 20.0);
    }

    #[test]
    fn test_set_mode() {
        let mut h = handler(OverflowMode::Clip, 50.0);
        h.set_mode(OverflowMode::Ellipsis);
        assert_eq!(h.mode, OverflowMode::Ellipsis);
    }

    #[test]
    fn test_set_max_width() {
        let mut h = handler(OverflowMode::Clip, 50.0);
        h.set_max_width(100.0);
        assert_eq!(h.max_width, 100.0);
    }

    #[test]
    fn test_set_ellipsis() {
        let mut h = handler(OverflowMode::Ellipsis, 30.0);
        h.set_ellipsis("...");
        let result = h.truncate("abcdefghij");
        assert!(result.ends_with("..."));
    }

    #[test]
    fn test_clip_empty() {
        let h = handler(OverflowMode::Clip, 50.0);
        assert_eq!(h.truncate(""), "");
    }

    #[test]
    fn test_zero_max_width() {
        let h = handler(OverflowMode::Ellipsis, 0.0);
        assert_eq!(h.truncate("anything"), "");
    }

    #[test]
    fn test_negative_max_width() {
        let h = handler(OverflowMode::Ellipsis, -10.0);
        assert_eq!(h.truncate("anything"), "");
    }
}
