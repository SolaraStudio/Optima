#[derive(Debug, Clone)]
pub struct ResizeEvent {
    pub old_width: u32,
    pub old_height: u32,
    pub new_width: u32,
    pub new_height: u32,
    pub target: Option<String>,
}

impl ResizeEvent {
    pub fn new(old_width: u32, old_height: u32, new_width: u32, new_height: u32) -> Self {
        ResizeEvent {
            old_width,
            old_height,
            new_width,
            new_height,
            target: None,
        }
    }

    pub fn with_target(mut self, target: &str) -> Self {
        self.target = Some(target.to_string());
        self
    }

    pub fn width_changed(&self) -> bool {
        self.old_width != self.new_width
    }

    pub fn height_changed(&self) -> bool {
        self.old_height != self.new_height
    }

    pub fn both_changed(&self) -> bool {
        self.width_changed() && self.height_changed()
    }
}
