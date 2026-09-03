#[derive(Debug, Clone)]
pub struct ScrollEvent {
    pub scroll_x: f32,
    pub scroll_y: f32,
    pub max_scroll_x: f32,
    pub max_scroll_y: f32,
    pub delta_x: f32,
    pub delta_y: f32,
    pub target: Option<String>,
}

impl ScrollEvent {
    pub fn new(scroll_x: f32, scroll_y: f32) -> Self {
        ScrollEvent {
            scroll_x,
            scroll_y,
            max_scroll_x: 0.0,
            max_scroll_y: 0.0,
            delta_x: 0.0,
            delta_y: 0.0,
            target: None,
        }
    }

    pub fn with_max_scroll(mut self, max_x: f32, max_y: f32) -> Self {
        self.max_scroll_x = max_x;
        self.max_scroll_y = max_y;
        self
    }

    pub fn with_delta(mut self, dx: f32, dy: f32) -> Self {
        self.delta_x = dx;
        self.delta_y = dy;
        self
    }

    pub fn with_target(mut self, target: &str) -> Self {
        self.target = Some(target.to_string());
        self
    }

    pub fn is_at_left(&self) -> bool {
        self.scroll_x <= 0.0
    }

    pub fn is_at_right(&self) -> bool {
        self.scroll_x >= self.max_scroll_x
    }

    pub fn is_at_top(&self) -> bool {
        self.scroll_y <= 0.0
    }

    pub fn is_at_bottom(&self) -> bool {
        self.scroll_y >= self.max_scroll_y
    }
}
