use crate::layout::fragment::Fragment;

#[derive(Debug, Clone, Default)]
pub struct LineBox {
    pub y: f32,
    pub height: f32,
    pub fragments: Vec<Fragment>,
    pub baseline: f32,
}

impl LineBox {
    pub fn new(y: f32) -> Self {
        LineBox {
            y,
            ..Default::default()
        }
    }

    pub fn add_fragment(&mut self, frag: Fragment) {
        self.height = self.height.max(frag.height);
        self.fragments.push(frag);
    }

    pub fn total_width(&self) -> f32 {
        self.fragments.iter().map(|f| f.width).sum()
    }
}
