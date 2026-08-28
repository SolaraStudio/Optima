use taffy::prelude::*;

pub struct GridLayout;

impl GridLayout {
    pub fn layout(
        children: &[taffy::Node],
        columns: u32,
        rows: u32,
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
                display: Display::Grid,
                grid_template: GridTrackVec {
                    columns: vec![GridTrack::auto(); columns as usize],
                    rows: vec![GridTrack::auto(); rows as usize],
                },
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

    pub fn layout_with_styles(
        nodes: &[taffy::Node],
        columns: Vec<GridTrack>,
        rows: Vec<GridTrack>,
        container_width: f32,
    ) -> Vec<taffy::Layout> {
        let mut tree = Taffy::new();
        let container = tree.new_node(
            Style {
                display: Display::Grid,
                grid_template: GridTrackVec {
                    columns,
                    rows,
                },
                size: Size {
                    width: Dimension::Points(container_width),
                    height: Dimension::Auto,
                },
                ..Default::default()
            },
            nodes.to_vec(),
        ).unwrap();
        tree.compute_layout(container, Size { width: AvailableSpace::Definite(container_width), height: AvailableSpace::Auto }).unwrap();
        nodes.iter().map(|n| tree.layout(*n).unwrap()).collect()
    }

    pub fn layout_with_gap(
        children: &[taffy::Node],
        columns: u32,
        rows: u32,
        gap: Size<f32>,
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
                display: Display::Grid,
                grid_template: GridTrackVec {
                    columns: vec![GridTrack::auto(); columns as usize],
                    rows: vec![GridTrack::auto(); rows as usize],
                },
                gap,
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
