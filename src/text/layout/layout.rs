use crate::text::line::TextLine;
use crate::text::run::TextRun;

#[derive(Debug, Clone)]
pub struct TextLayout {
    pub runs: Vec<TextRun>,
    pub lines: Vec<TextLine>,
    pub width: f32,
    pub height: f32,
}

impl Default for TextLayout {
    fn default() -> Self {
        Self::new()
    }
}

impl TextLayout {
    pub fn new() -> Self {
        TextLayout {
            runs: Vec::new(),
            lines: Vec::new(),
            width: 0.0,
            height: 0.0,
        }
    }

    pub fn add_run(&mut self, run: TextRun) {
        self.runs.push(run);
    }

    pub fn layout(&mut self, max_width: f32, font_size: f32) {
        self.lines.clear();
        let line_height = font_size * 1.2;
        let mut current_line = TextLine::new(0.0, 0.0, max_width);
        for run in &self.runs {
            let mut x = current_line.width();
            for glyph in &run.glyphs {
                if x + glyph.advance_x > max_width && x > 0.0 {
                    self.lines.push(current_line.clone());
                    current_line =
                        TextLine::new(0.0, self.lines.len() as f32 * line_height, max_width);
                    x = 0.0;
                }
                current_line.add_glyph(glyph.clone(), x);
                x += glyph.advance_x;
            }
        }
        if current_line.has_glyphs() {
            self.lines.push(current_line);
        }
        self.height = self.lines.len() as f32 * line_height;
        self.width = self.lines.iter().map(|l| l.width()).fold(0.0f32, f32::max);
    }

    pub fn hit_test(&self, x: f32, y: f32) -> Option<usize> {
        for line in &self.lines {
            if y >= line.y && y <= line.y + line.height {
                return line.hit_test(x);
            }
        }
        None
    }
}
