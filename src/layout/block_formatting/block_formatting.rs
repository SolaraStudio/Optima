use crate::css::computed::ComputedStyle;
use crate::layout::block::BlockLayout;
use crate::layout::fragment::Fragment;

pub struct BlockFormattingContext;

impl BlockFormattingContext {
    pub fn layout(
        styles: &[(usize, ComputedStyle)],
        available_width: f32,
        base_x: f32,
        base_y: f32,
    ) -> Vec<Fragment> {
        let mut fragments = Vec::new();
        let mut y = base_y;
        for (node_id, style) in styles {
            let bm = BlockLayout::layout(style, available_width, 0.0);
            let mut frag = Fragment::new();
            frag.node_id = Some(*node_id);
            frag.x = base_x + bm.margin_left;
            frag.y = y + bm.margin_top;
            frag.width = bm.content_width;
            frag.height = bm.content_height.max(20.0);
            y = frag.y + frag.height + bm.margin_bottom;
            fragments.push(frag);
        }
        fragments
    }
}
