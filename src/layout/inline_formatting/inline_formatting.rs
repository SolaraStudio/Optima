use crate::layout::fragment::Fragment;

pub struct InlineFormattingContext;

impl InlineFormattingContext {
    pub fn layout_line_box(fragments: Vec<Fragment>, _available_width: f32) -> Vec<Fragment> {
        let mut line_x = 0.0f32;
        let mut result = Vec::new();
        let mut line_y = 0.0f32;
        let mut line_height = 0.0f32;
        for mut frag in fragments {
            if line_x + frag.width > _available_width && line_x > 0.0 {
                line_y += line_height;
                line_x = 0.0;
                line_height = 0.0;
            }
            frag.x = line_x;
            frag.y = line_y;
            line_x += frag.width;
            line_height = line_height.max(frag.height);
            result.push(frag);
        }
        result
    }
}
