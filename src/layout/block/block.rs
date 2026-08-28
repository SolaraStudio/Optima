use taffy::prelude::*;

pub struct BlockLayout;

impl BlockLayout {
    pub fn layout(children: &[taffy::Node], container_width: f32) -> Vec<taffy::Layout> {
        let mut tree = Taffy::new();
        let mut nodes = Vec::new();
        for child in children {
            let style = Style {
                size: Size {
                    width: Dimension::Percent(100.0),
                    height: Dimension::Auto,
                },
                ..Default::default()
            };
            let node = tree.new_node(style, Vec::new()).unwrap();
            nodes.push(node);
        }
        let container = tree.new_node(
            Style {
                size: Size {
                    width: Dimension::Points(container_width),
                    height: Dimension::Auto,
                },
                flex_direction: FlexDirection::Column,
                ..Default::default()
            },
            nodes.clone(),
        ).unwrap();
        tree.compute_layout(container, Size { width: AvailableSpace::Definite(container_width), height: AvailableSpace::Auto }).unwrap();
        nodes.iter().map(|n| tree.layout(*n).unwrap()).collect()
    }
}
