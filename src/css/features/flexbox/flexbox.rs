#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FlexDirection {
    Row,
    RowReverse,
    Column,
    ColumnReverse,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FlexWrap {
    NoWrap,
    Wrap,
    WrapReverse,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum JustifyContent {
    FlexStart,
    FlexEnd,
    Center,
    SpaceBetween,
    SpaceAround,
    SpaceEvenly,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AlignItems {
    FlexStart,
    FlexEnd,
    Center,
    Baseline,
    Stretch,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AlignSelf {
    Auto,
    FlexStart,
    FlexEnd,
    Center,
    Baseline,
    Stretch,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AlignContent {
    FlexStart,
    FlexEnd,
    Center,
    SpaceBetween,
    SpaceAround,
    Stretch,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FlexBasis {
    Auto,
    Content,
    Fixed(f32),
}

#[derive(Debug, Clone)]
pub struct FlexItem {
    pub width: f32,
    pub height: f32,
    pub min_width: Option<f32>,
    pub max_width: Option<f32>,
    pub min_height: Option<f32>,
    pub max_height: Option<f32>,
    pub flex_grow: f32,
    pub flex_shrink: f32,
    pub flex_basis: FlexBasis,
    pub align_self: AlignSelf,
    pub order: i32,
}

impl FlexItem {
    pub fn new() -> Self {
        FlexItem {
            width: 0.0,
            height: 0.0,
            min_width: None,
            max_width: None,
            min_height: None,
            max_height: None,
            flex_grow: 0.0,
            flex_shrink: 1.0,
            flex_basis: FlexBasis::Auto,
            align_self: AlignSelf::Auto,
            order: 0,
        }
    }

    pub fn with_grow(mut self, grow: f32) -> Self {
        self.flex_grow = grow;
        self
    }

    pub fn with_shrink(mut self, shrink: f32) -> Self {
        self.flex_shrink = shrink;
        self
    }

    pub fn with_basis(mut self, basis: FlexBasis) -> Self {
        self.flex_basis = basis;
        self
    }

    pub fn clamp_width(&self, w: f32) -> f32 {
        let min = self.min_width.unwrap_or(0.0);
        let max = self.max_width.unwrap_or(f32::INFINITY);
        w.clamp(min, max)
    }

    pub fn clamp_height(&self, h: f32) -> f32 {
        let min = self.min_height.unwrap_or(0.0);
        let max = self.max_height.unwrap_or(f32::INFINITY);
        h.clamp(min, max)
    }
}

#[derive(Debug, Clone)]
pub struct FlexContainer {
    pub direction: FlexDirection,
    pub wrap: FlexWrap,
    pub justify_content: JustifyContent,
    pub align_items: AlignItems,
    pub align_content: AlignContent,
    pub gap: f32,
    pub width: f32,
    pub height: f32,
}

impl FlexContainer {
    pub fn new() -> Self {
        FlexContainer {
            direction: FlexDirection::Row,
            wrap: FlexWrap::NoWrap,
            justify_content: JustifyContent::FlexStart,
            align_items: AlignItems::Stretch,
            align_content: AlignContent::Stretch,
            gap: 0.0,
            width: 0.0,
            height: 0.0,
        }
    }

    pub fn with_direction(mut self, direction: FlexDirection) -> Self {
        self.direction = direction;
        self
    }

    pub fn with_wrap(mut self, wrap: FlexWrap) -> Self {
        self.wrap = wrap;
        self
    }

    pub fn with_justify(mut self, justify: JustifyContent) -> Self {
        self.justify_content = justify;
        self
    }

    pub fn with_align_items(mut self, align: AlignItems) -> Self {
        self.align_items = align;
        self
    }

    pub fn with_gap(mut self, gap: f32) -> Self {
        self.gap = gap;
        self
    }

    pub fn is_main_axis_horizontal(&self) -> bool {
        matches!(
            self.direction,
            FlexDirection::Row | FlexDirection::RowReverse
        )
    }

    pub fn main_size(&self) -> f32 {
        if self.is_main_axis_horizontal() {
            self.width
        } else {
            self.height
        }
    }

    pub fn cross_size(&self) -> f32 {
        if self.is_main_axis_horizontal() {
            self.height
        } else {
            self.width
        }
    }

    pub fn layout(&self, items: &[FlexItem]) -> Vec<LayoutBox> {
        if items.is_empty() {
            return Vec::new();
        }
        let main_size = self.main_size();
        let gap_count = (items.len() as f32 - 1.0).max(0.0);
        let total_gap = gap_count * self.gap;
        let available_main = main_size - total_gap;
        let total_flex_grow: f32 = items.iter().map(|i| i.flex_grow).sum();
        let mut allocated_main: Vec<f32> = items
            .iter()
            .map(|item| match item.flex_basis {
                FlexBasis::Auto => {
                    if self.is_main_axis_horizontal() {
                        item.width
                    } else {
                        item.height
                    }
                }
                FlexBasis::Content => {
                    if self.is_main_axis_horizontal() {
                        item.width
                    } else {
                        item.height
                    }
                }
                FlexBasis::Fixed(v) => v,
            })
            .collect();
        let total_base: f32 = allocated_main.iter().sum();
        let remaining = available_main - total_base;
        if remaining > 0.0 && total_flex_grow > 0.0 {
            for (i, item) in items.iter().enumerate() {
                let extra = remaining * (item.flex_grow / total_flex_grow);
                allocated_main[i] += extra;
            }
        } else if remaining < 0.0 {
            let total_shrink: f32 = items.iter().map(|i| i.flex_shrink).sum();
            if total_shrink > 0.0 {
                for (i, item) in items.iter().enumerate() {
                    let shrink = remaining * (item.flex_shrink / total_shrink);
                    allocated_main[i] = (allocated_main[i] + shrink).max(0.0);
                }
            }
        }
        for (i, item) in items.iter().enumerate() {
            allocated_main[i] = item.clamp_width(allocated_main[i]);
        }
        let mut positions: Vec<f32> = Vec::new();
        let mut cursor = self.compute_start_offset(available_main, &allocated_main);
        let reversed = matches!(
            self.direction,
            FlexDirection::RowReverse | FlexDirection::ColumnReverse
        );
        for (_i, &size) in allocated_main.iter().enumerate() {
            positions.push(cursor);
            if reversed {
                cursor -= size + self.gap;
            } else {
                cursor += size + self.gap;
            }
        }
        let cross_size = self.cross_size();
        let mut boxes = Vec::new();
        for (i, item) in items.iter().enumerate() {
            let item_cross = match self.align_items {
                AlignItems::Stretch => cross_size,
                _ => {
                    if self.is_main_axis_horizontal() {
                        item.height
                    } else {
                        item.width
                    }
                }
            };
            let x;
            let y;
            if self.is_main_axis_horizontal() {
                x = positions[i];
                y = self.compute_cross_offset(item_cross, cross_size);
            } else {
                x = self.compute_cross_offset(item_cross, cross_size);
                y = positions[i];
            }
            let effective_width = if self.is_main_axis_horizontal() {
                allocated_main[i]
            } else {
                item_cross
            };
            let effective_height = if self.is_main_axis_horizontal() {
                item_cross
            } else {
                allocated_main[i]
            };
            boxes.push(LayoutBox {
                x,
                y,
                width: effective_width,
                height: effective_height,
            });
        }
        boxes
    }

    fn compute_start_offset(&self, available: f32, sizes: &[f32]) -> f32 {
        let total: f32 = sizes.iter().sum();
        let gap_total = (sizes.len() as f32 - 1.0).max(0.0) * self.gap;
        let free = available - total - gap_total;
        match self.justify_content {
            JustifyContent::FlexStart => 0.0,
            JustifyContent::FlexEnd => free.max(0.0),
            JustifyContent::Center => free.max(0.0) / 2.0,
            JustifyContent::SpaceBetween => 0.0,
            JustifyContent::SpaceAround => {
                let gap_count = sizes.len() as f32;
                let space = if gap_count > 0.0 { free / gap_count } else { 0.0 };
                space / 2.0
            }
            JustifyContent::SpaceEvenly => {
                let gap_count = sizes.len() as f32 + 1.0;
                let space = if gap_count > 0.0 { free / gap_count } else { 0.0 };
                space
            }
        }
    }

    fn compute_cross_offset(&self, item_cross: f32, container_cross: f32) -> f32 {
        match self.align_items {
            AlignItems::FlexStart => 0.0,
            AlignItems::FlexEnd => (container_cross - item_cross).max(0.0),
            AlignItems::Center => (container_cross - item_cross).max(0.0) / 2.0,
            AlignItems::Baseline => 0.0,
            AlignItems::Stretch => 0.0,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct LayoutBox {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flex_item_clamp() {
        let item = FlexItem {
            min_width: Some(50.0),
            max_width: Some(200.0),
            ..FlexItem::new()
        };
        assert_eq!(item.clamp_width(10.0), 50.0);
        assert_eq!(item.clamp_width(100.0), 100.0);
        assert_eq!(item.clamp_width(300.0), 200.0);
    }

    #[test]
    fn test_flex_item_builder() {
        let item = FlexItem::new()
            .with_grow(1.0)
            .with_shrink(0.0)
            .with_basis(FlexBasis::Fixed(100.0));
        assert_eq!(item.flex_grow, 1.0);
        assert_eq!(item.flex_shrink, 0.0);
        assert_eq!(item.flex_basis, FlexBasis::Fixed(100.0));
    }

    #[test]
    fn test_container_properties() {
        let container = FlexContainer::new()
            .with_direction(FlexDirection::Column)
            .with_wrap(FlexWrap::Wrap)
            .with_justify(JustifyContent::Center)
            .with_align_items(AlignItems::FlexEnd)
            .with_gap(10.0);
        assert_eq!(container.direction, FlexDirection::Column);
        assert_eq!(container.wrap, FlexWrap::Wrap);
        assert_eq!(container.justify_content, JustifyContent::Center);
        assert_eq!(container.align_items, AlignItems::FlexEnd);
        assert_eq!(container.gap, 10.0);
        assert!(!container.is_main_axis_horizontal());
    }

    #[test]
    fn test_layout_simple_row() {
        let container = FlexContainer {
            width: 300.0,
            height: 100.0,
            direction: FlexDirection::Row,
            gap: 0.0,
            ..FlexContainer::new()
        };
        let items = vec![
            FlexItem { width: 100.0, height: 50.0, ..FlexItem::new() },
            FlexItem { width: 100.0, height: 50.0, ..FlexItem::new() },
        ];
        let boxes = container.layout(&items);
        assert_eq!(boxes.len(), 2);
        assert_eq!(boxes[0].x, 0.0);
        assert_eq!(boxes[1].x, 100.0);
    }

    #[test]
    fn test_layout_with_gap() {
        let container = FlexContainer {
            width: 300.0,
            height: 100.0,
            direction: FlexDirection::Row,
            gap: 10.0,
            ..FlexContainer::new()
        };
        let items = vec![
            FlexItem { width: 100.0, height: 50.0, ..FlexItem::new() },
            FlexItem { width: 100.0, height: 50.0, ..FlexItem::new() },
        ];
        let boxes = container.layout(&items);
        assert_eq!(boxes[0].x, 0.0);
        assert_eq!(boxes[1].x, 110.0);
    }

    #[test]
    fn test_layout_with_flex_grow() {
        let container = FlexContainer {
            width: 300.0,
            height: 100.0,
            direction: FlexDirection::Row,
            gap: 0.0,
            ..FlexContainer::new()
        };
        let items = vec![
            FlexItem { width: 50.0, flex_grow: 1.0, ..FlexItem::new() },
            FlexItem { width: 50.0, flex_grow: 2.0, ..FlexItem::new() },
        ];
        let boxes = container.layout(&items);
        assert_eq!(boxes[0].width, 116.666664);
        assert_eq!(boxes[1].width, 183.33333);
    }

    #[test]
    fn test_layout_column() {
        let container = FlexContainer {
            width: 100.0,
            height: 300.0,
            direction: FlexDirection::Column,
            gap: 0.0,
            ..FlexContainer::new()
        };
        let items = vec![
            FlexItem { width: 50.0, height: 100.0, ..FlexItem::new() },
            FlexItem { width: 50.0, height: 100.0, ..FlexItem::new() },
        ];
        let boxes = container.layout(&items);
        assert_eq!(boxes[0].y, 0.0);
        assert_eq!(boxes[1].y, 100.0);
    }

    #[test]
    fn test_layout_center_justify() {
        let container = FlexContainer {
            width: 300.0,
            height: 100.0,
            direction: FlexDirection::Row,
            justify_content: JustifyContent::Center,
            gap: 0.0,
            ..FlexContainer::new()
        };
        let items = vec![
            FlexItem { width: 100.0, height: 50.0, ..FlexItem::new() },
        ];
        let boxes = container.layout(&items);
        assert_eq!(boxes[0].x, 100.0);
    }

    #[test]
    fn test_layout_align_items_center() {
        let container = FlexContainer {
            width: 300.0,
            height: 100.0,
            direction: FlexDirection::Row,
            align_items: AlignItems::Center,
            gap: 0.0,
            ..FlexContainer::new()
        };
        let items = vec![
            FlexItem { width: 50.0, height: 40.0, ..FlexItem::new() },
        ];
        let boxes = container.layout(&items);
        assert_eq!(boxes[0].y, 30.0);
    }

    #[test]
    fn test_empty_layout() {
        let container = FlexContainer::new();
        let boxes = container.layout(&[]);
        assert!(boxes.is_empty());
    }
}
