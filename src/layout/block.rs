use taffy::prelude::*;

pub struct BlockLayout {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl BlockLayout {
    pub fn layout(children: &[taffy::Node]) -> Vec<Self> {
        let mut tree = Taffy::new();
        let mut nodes = Vec::new();
        for _ in children {
            let node = tree.new_leaf(Style {
                size: Size { width: Dimension::Auto, height: Dimension::Auto },
                ..Default::default()
            }).unwrap();
            nodes.push(node);
        }
        let container = tree.new_leaf(Style {
            size: Size { width: Dimension::Percent(1.0), height: Dimension::Auto },
            ..Default::default()
        }).unwrap();
        tree.compute_layout(container, Size { width: AvailableSpace::Definite(800.0), height: AvailableSpace::Auto }).unwrap();
        let mut result = Vec::new();
        for node in &nodes {
            let layout = tree.layout(*node).unwrap();
            result.push(BlockLayout {
                x: layout.location.x,
                y: layout.location.y,
                width: layout.size.width,
                height: layout.size.height,
            });
        }
        result
    }
}
