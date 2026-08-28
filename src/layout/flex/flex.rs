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
}
