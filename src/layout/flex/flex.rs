use taffy::prelude::*;

pub struct FlexLayout;

impl FlexLayout {
    pub fn layout(
        children: &[taffy::Node],
        direction: FlexDirection,
        gap: f32,
        container_width: f32,
    ) -> Vec<taffy::Layout> {
        let mut tree = Taffy::new();
        let mut nodes = Vec::new();
        for child in children {
            let node = tree.new_leaf(Style::default()).unwrap();
            nodes.push(node);
        }
        let container = tree.new_node(
            Style {
                flex_direction: direction,
                gap: Size { width: gap, height: gap },
                size: Size {
                    width: Dimension::Points(container_width),
                    height: Dimension::Auto,
                },
                ..Default::default()
            },
            nodes.clone(),
        ).unwrap();
        tree.compute_layout(container, Size { width: AvailableSpace::Definite(container_width), height: AvailableSpace::Auto }).unwrap();
        nodes.iter().map(|n| tree.layout(*n).unwrap()).collect()
    }

    pub fn layout_with_align(
        children: &[taffy::Node],
        direction: FlexDirection,
        gap: f32,
        align_items: AlignItems,
        justify_content: JustifyContent,
        container_width: f32,
    ) -> Vec<taffy::Layout> {
        let mut tree = Taffy::new();
        let mut nodes = Vec::new();
        for child in children {
            let node = tree.new_leaf(Style::default()).unwrap();
            nodes.push(node);
        }
        let container = tree.new_node(
            Style {
                flex_direction: direction,
                gap: Size { width: gap, height: gap },
                align_items,
                justify_content,
                size: Size {
                    width: Dimension::Points(container_width),
                    height: Dimension::Auto,
                },
                ..Default::default()
            },
            nodes.clone(),
        ).unwrap();
        tree.compute_layout(container, Size { width: AvailableSpace::Definite(container_width), height: AvailableSpace::Auto }).unwrap();
        nodes.iter().map(|n| tree.layout(*n).unwrap()).collect()
    }

    pub fn layout_with_flex(
        children: &[(taffy::Node, f32)],
        direction: FlexDirection,
        gap: f32,
        container_width: f32,
    ) -> Vec<taffy::Layout> {
        let mut tree = Taffy::new();
        let mut nodes = Vec::new();
        for (child, flex) in children {
            let style = Style {
                flex_grow: *flex,
                flex_shrink: 1.0,
                size: Size {
                    width: Dimension::Auto,
                    height: Dimension::Auto,
                },
                ..Default::default()
            };
            let node = tree.new_node(style, Vec::new()).unwrap();
            nodes.push(node);
        }
        let container = tree.new_node(
            Style {
                flex_direction: direction,
                gap: Size { width: gap, height: gap },
                size: Size {
                    width: Dimension::Points(container_width),
                    height: Dimension::Auto,
                },
                ..Default::default()
            },
            nodes.clone(),
        ).unwrap();
        tree.compute_layout(container, Size { width: AvailableSpace::Definite(container_width), height: AvailableSpace::Auto }).unwrap();
        nodes.iter().map(|n| tree.layout(*n).unwrap()).collect()
    }
}
