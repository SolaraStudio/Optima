use crate::layout::box_model::BoxModel;
use crate::css::computed::ComputedStyle;

pub struct BlockLayout;

impl BlockLayout {
    pub fn layout(style: &ComputedStyle, width: f32, _height: f32) -> BoxModel {
        let mut bm = BoxModel::new();
        Self::resolve_margins(&mut bm, style);
        Self::resolve_padding(&mut bm, style);
        Self::resolve_border(&mut bm, style);
        bm.content_width = width - bm.padding_left - bm.padding_right - bm.border_left - bm.border_right;
        bm
    }

    fn resolve_margins(bm: &mut BoxModel, style: &ComputedStyle) {
        if let Some(v) = style.get("margin-top").and_then(|v| v.as_length()) {
            bm.margin_top = v.to_px(16.0);
        }
        if let Some(v) = style.get("margin-bottom").and_then(|v| v.as_length()) {
            bm.margin_bottom = v.to_px(16.0);
        }
        if let Some(v) = style.get("margin-left").and_then(|v| v.as_length()) {
            bm.margin_left = v.to_px(16.0);
        }
        if let Some(v) = style.get("margin-right").and_then(|v| v.as_length()) {
            bm.margin_right = v.to_px(16.0);
        }
    }

    fn resolve_padding(bm: &mut BoxModel, style: &ComputedStyle) {
        if let Some(v) = style.get("padding-top").and_then(|v| v.as_length()) {
            bm.padding_top = v.to_px(16.0);
        }
        if let Some(v) = style.get("padding-bottom").and_then(|v| v.as_length()) {
            bm.padding_bottom = v.to_px(16.0);
        }
        if let Some(v) = style.get("padding-left").and_then(|v| v.as_length()) {
            bm.padding_left = v.to_px(16.0);
        }
        if let Some(v) = style.get("padding-right").and_then(|v| v.as_length()) {
            bm.padding_right = v.to_px(16.0);
        }
    }

    fn resolve_border(bm: &mut BoxModel, style: &ComputedStyle) {
        if let Some(v) = style.get("border-top-width").and_then(|v| v.as_length()) {
            bm.border_top = v.to_px(16.0);
        }
        if let Some(v) = style.get("border-bottom-width").and_then(|v| v.as_length()) {
            bm.border_bottom = v.to_px(16.0);
        }
        if let Some(v) = style.get("border-left-width").and_then(|v| v.as_length()) {
            bm.border_left = v.to_px(16.0);
        }
        if let Some(v) = style.get("border-right-width").and_then(|v| v.as_length()) {
            bm.border_right = v.to_px(16.0);
        }
    }
}
