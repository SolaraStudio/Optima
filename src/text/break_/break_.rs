pub struct TextBreaker;

impl TextBreaker {
    pub fn break_text(text: &str, max_width: f32, font_size: f32) -> Vec<String> {
        let mut lines = Vec::new();
        let mut current_line = String::new();
        let mut current_width = 0.0;
        let chars: Vec<char> = text.chars().collect();
        let mut i = 0;
        while i < chars.len() {
            let c = chars[i];
            let char_width = Self::char_width(c, font_size);
            if current_width + char_width > max_width {
                if !current_line.is_empty() {
                    lines.push(current_line);
                    current_line = String::new();
                    current_width = 0.0;
                }
            }
            current_line.push(c);
            current_width += char_width;
            i += 1;
        }
        if !current_line.is_empty() {
            lines.push(current_line);
        }
        lines
    }

    pub fn char_width(c: char, font_size: f32) -> f32 {
        match c {
            ' ' => font_size * 0.3,
            'I' | 'i' | 'l' | '1' | '|' => font_size * 0.3,
            'M' | 'W' | 'w' => font_size * 0.8,
            _ => font_size * 0.5,
        }
    }

    pub fn measure_line(text: &str, font_size: f32) -> f32 {
        text.chars().map(|c| Self::char_width(c, font_size)).sum()
    }

    pub fn split_at_word_boundary(text: &str, max_width: f32, font_size: f32) -> (String, String) {
        let chars: Vec<char> = text.chars().collect();
        let mut width = 0.0;
        let mut last_space = 0;
        let mut i = 0;
        while i < chars.len() {
            let c = chars[i];
            let char_width = Self::char_width(c, font_size);
            if c == ' ' {
                last_space = i;
            }
            if width + char_width > max_width {
                if last_space > 0 {
                    return (chars[0..last_space].iter().collect(), chars[last_space..].iter().collect());
                } else {
                    return (chars[0..i].iter().collect(), chars[i..].iter().collect());
                }
            }
            width += char_width;
            i += 1;
        }
        (text.to_string(), String::new())
    }
}
