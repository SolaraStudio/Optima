use crate::css::computed::ComputedStyle;

pub struct InlineLayout;

impl InlineLayout {
    pub fn layout_inline_box(style: &ComputedStyle, x: f32, y: f32, _width: f32) -> (f32, f32) {
        let mut dx = 0.0f32;
        if let Some(v) = style.get("margin-left").and_then(|v| v.as_length()) {
            dx += v.to_px(16.0);
        }
        if let Some(v) = style.get("padding-left").and_then(|v| v.as_length()) {
            dx += v.to_px(16.0);
        }
        (x + dx, y)
    }
}
