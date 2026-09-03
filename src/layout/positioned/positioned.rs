use crate::layout::fragment::Fragment;
use crate::css::computed::ComputedStyle;

pub struct PositionedLayout;

impl PositionedLayout {
    pub fn resolve_position(style: &ComputedStyle, x: f32, y: f32, w: f32, h: f32, containing_w: f32, containing_h: f32) -> (f32, f32) {
        let position = style.get("position").and_then(|v| v.as_string()).unwrap_or("static");
        match position {
            "absolute" => {
                let mut rx = x;
                let mut ry = y;
                if let Some(v) = style.get("left").and_then(|v| v.as_length()) { rx = v.to_px(containing_w); }
                if let Some(v) = style.get("top").and_then(|v| v.as_length()) { ry = v.to_px(containing_h); }
                if let Some(v) = style.get("right").and_then(|v| v.as_length()) { rx = containing_w - w - v.to_px(containing_w); }
                if let Some(v) = style.get("bottom").and_then(|v| v.as_length()) { ry = containing_h - h - v.to_px(containing_h); }
                (rx, ry)
            }
            "fixed" => {
                let mut rx = x;
                let mut ry = y;
                if let Some(v) = style.get("left").and_then(|v| v.as_length()) { rx = v.to_px(containing_w); }
                if let Some(v) = style.get("top").and_then(|v| v.as_length()) { ry = v.to_px(containing_h); }
                (rx, ry)
            }
            "sticky" => (x, y.max(0.0)),
            _ => (x, y),
        }
    }
}
