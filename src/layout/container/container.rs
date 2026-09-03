use crate::layout::box_model::BoxModel;
use crate::layout::fragment::Fragment;
use crate::css::computed::ComputedStyle;

pub struct ContainerLayout;

impl ContainerLayout {
    pub fn compute_content_rect(style: &ComputedStyle, bm: &BoxModel) -> (f32, f32, f32, f32) {
        let display = style.get("display").and_then(|v| v.as_string()).unwrap_or("block");
        match display {
            "flex" | "inline-flex" => (0.0, 0.0, bm.content_width, bm.content_height),
            "grid" | "inline-grid" => (0.0, 0.0, bm.content_width, bm.content_height),
            _ => (0.0, 0.0, bm.content_width, bm.content_height),
        }
    }
}
