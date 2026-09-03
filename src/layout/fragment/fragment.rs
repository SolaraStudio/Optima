#[derive(Debug, Clone, Default)]
pub struct Fragment {
    pub node_id: Option<usize>,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub children: Vec<Fragment>,
}

impl Fragment {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_position(mut self, x: f32, y: f32) -> Self {
        self.x = x;
        self.y = y;
        self
    }

    pub fn with_size(mut self, w: f32, h: f32) -> Self {
        self.width = w;
        self.height = h;
        self
    }

    pub fn with_node(mut self, id: usize) -> Self {
        self.node_id = Some(id);
        self
    }

    pub fn add_child(&mut self, child: Fragment) {
        self.children.push(child);
    }

    pub fn hit_test(&self, x: f32, y: f32) -> Option<usize> {
        if x >= self.x && x <= self.x + self.width && y >= self.y && y <= self.y + self.height {
            for child in &self.children {
                if let Some(id) = child.hit_test(x, y) {
                    return Some(id);
                }
            }
            return self.node_id;
        }
        None
    }
}
