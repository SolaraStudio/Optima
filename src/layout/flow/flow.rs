use crate::layout::fragment::Fragment;

#[derive(Debug, Clone, Default)]
pub struct FlowContext {
    pub x: f32,
    pub y: f32,
    pub available_width: f32,
    pub fragments: Vec<Fragment>,
}

impl FlowContext {
    pub fn new(x: f32, y: f32, available_width: f32) -> Self {
        FlowContext { x, y, available_width, fragments: Vec::new() }
    }

    pub fn add_fragment(&mut self, fragment: Fragment) {
        self.fragments.push(fragment);
    }

    pub fn total_height(&self) -> f32 {
        self.fragments.iter().map(|f| f.y + f.height).fold(0.0f32, f32::max)
    }

    pub fn advance_y(&mut self, amount: f32) { self.y += amount; }
}
