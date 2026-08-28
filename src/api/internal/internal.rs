use crate::dom::Document;
use crate::css::ComputedStyle;
use crate::layout::block::BlockLayout;

pub struct InternalAPI;

impl InternalAPI {
    pub fn resolve_styles(document: &Document) -> Vec<ComputedStyle> {
        let mut styles = Vec::new();
        let mut stack = vec![&document.root];
        while let Some(node) = stack.pop() {
            if let Some(data) = &node.element_data {
                let style = ComputedStyle::new();
                styles.push(style);
            }
            for child in &node.children {
                stack.push(child);
            }
        }
        styles
    }

    pub fn compute_layout(children: &[taffy::Node], container_width: f32) -> Vec<taffy::Layout> {
        BlockLayout::layout(children, container_width)
    }
}
