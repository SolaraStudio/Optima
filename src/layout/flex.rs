use taffy::prelude::*;

pub struct FlexLayout {
    pub direction: taffy::FlexDirection,
    pub gap: f32,
}

impl FlexLayout {
    pub fn layout(children: &[taffy::Node], direction: taffy::FlexDirection, gap: f32) -> Vec<taffy::Layout> {
        let mut tree = Taffy::new();
        let mut nodes = Vec::new();
        for _ in children {
            let node = tree.new_leaf(Style {
                size: Size { width: Dimension::Auto, height: Dimension::Auto },
                ..Default::default()
            }).unwrap();
            nodes.push(node);
        }
        let container = tree.new_node(Style {
            flex_direction: direction,
            gap: Size { width: gap, height: gap },
            ..Default::default()
        }, nodes.clone()).unwrap();
        tree.compute_layout(container, Size { width: AvailableSpace::Definite(800.0), height: AvailableSpace::Auto }).unwrap();
        nodes.iter().map(|n| tree.layout(*n).unwrap()).collect()
    }
}
