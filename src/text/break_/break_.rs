#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BreakType {
    Soft,
    Hard,
    Hyphen,
    None,
}

pub struct LineBreaker;

impl LineBreaker {
    pub fn find_break(text: &str, max_width: f32, font_size: f32) -> (usize, BreakType) {
        let char_width = font_size * 0.6;
        let max_chars = (max_width / char_width) as usize;
        if text.len() <= max_chars {
            return (text.len(), BreakType::None);
        }
        let break_point = text[..max_chars].rfind(' ').unwrap_or(max_chars);
        (break_point, BreakType::Soft)
    }

    pub fn is_breakable(ch: char) -> bool {
        ch == ' ' || ch == '-' || ch == '\n' || ch == '\t'
    }
}
