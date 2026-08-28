use taffy::prelude::*;

pub struct GridLayout {
    pub columns: u32,
    pub rows: u32,
}

impl GridLayout {
    pub fn layout(children: &[taffy::Node], columns: u32, rows: u32) -> Vec<taffy::Layout> {
        let mut tree = Taffy::new();
        let mut nodes = Vec::new();
        for _ in children {
            let node = tree.new_leaf(Style::default()).unwrap();
            nodes.push(node);
        }
        let container = tree.new_node(Style {
            display: Display::Grid,
            grid_template: GridTrackVec {
                columns: vec![GridTrack::auto(); columns as usize],
                rows: vec![GridTrack::auto(); rows as usize],
            },
            ..Default::default()
        }, nodes.clone()).unwrap();
        tree.compute_layout(container, Size { width: AvailableSpace::Definite(800.0), height: AvailableSpace::Auto }).unwrap();
        nodes.iter().map(|n| tree.layout(*n).unwrap()).collect()
    }
}
