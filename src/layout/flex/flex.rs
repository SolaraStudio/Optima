use crate::layout::box_model::BoxModel;
use crate::layout::fragment::Fragment;
use crate::css::computed::ComputedStyle;

#[derive(Debug, Clone, PartialEq)]
pub enum FlexDirection { Row, RowReverse, Column, ColumnReverse }

#[derive(Debug, Clone, PartialEq)]
pub enum FlexWrap { NoWrap, Wrap, WrapReverse }

#[derive(Debug, Clone, PartialEq)]
pub enum JustifyContent { FlexStart, FlexEnd, Center, SpaceBetween, SpaceAround, SpaceEvenly }

#[derive(Debug, Clone, PartialEq)]
pub enum AlignItems { FlexStart, FlexEnd, Center, Stretch, Baseline }

#[derive(Debug, Clone, PartialEq)]
pub enum AlignContent { FlexStart, FlexEnd, Center, Stretch, SpaceBetween, SpaceAround }

pub struct FlexLayout;

impl FlexLayout {
    pub fn parse_direction(style: &ComputedStyle) -> FlexDirection {
        match style.get("flex-direction").and_then(|v| v.as_string()).unwrap_or("row") {
            "row-reverse" => FlexDirection::RowReverse,
            "column" => FlexDirection::Column,
            "column-reverse" => FlexDirection::ColumnReverse,
            _ => FlexDirection::Row,
        }
    }

    pub fn parse_wrap(style: &ComputedStyle) -> FlexWrap {
        match style.get("flex-wrap").and_then(|v| v.as_string()).unwrap_or("nowrap") {
            "wrap" => FlexWrap::Wrap,
            "wrap-reverse" => FlexWrap::WrapReverse,
            _ => FlexWrap::NoWrap,
        }
    }

    pub fn parse_justify(style: &ComputedStyle) -> JustifyContent {
        match style.get("justify-content").and_then(|v| v.as_string()).unwrap_or("flex-start") {
            "flex-end" => JustifyContent::FlexEnd,
            "center" => JustifyContent::Center,
            "space-between" => JustifyContent::SpaceBetween,
            "space-around" => JustifyContent::SpaceAround,
            "space-evenly" => JustifyContent::SpaceEvenly,
            _ => JustifyContent::FlexStart,
        }
    }

    pub fn parse_align(style: &ComputedStyle) -> AlignItems {
        match style.get("align-items").and_then(|v| v.as_string()).unwrap_or("stretch") {
            "flex-start" => AlignItems::FlexStart,
            "flex-end" => AlignItems::FlexEnd,
            "center" => AlignItems::Center,
            "baseline" => AlignItems::Baseline,
            _ => AlignItems::Stretch,
        }
    }

    pub fn compute_flex_grow(style: &ComputedStyle) -> f32 {
        style.get("flex-grow").and_then(|v| v.as_number()).unwrap_or(0.0)
    }

    pub fn compute_flex_shrink(style: &ComputedStyle) -> f32 {
        style.get("flex-shrink").and_then(|v| v.as_number()).unwrap_or(1.0)
    }

    pub fn compute_flex_basis(style: &ComputedStyle, container_width: f32) -> f32 {
        if let Some(v) = style.get("flex-basis").and_then(|v| v.as_length()) {
            v.to_px(container_width)
        } else if let Some(v) = style.get("width").and_then(|v| v.as_length()) {
            v.to_px(container_width)
        } else {
            0.0
        }
    }

    pub fn layout_item(bm: &BoxModel, x: f32, y: f32) -> Fragment {
        let mut frag = Fragment::new();
        frag.x = x + bm.margin_left;
        frag.y = y + bm.margin_top;
        frag.width = bm.content_width;
        frag.height = bm.content_height;
        frag
    }
}
