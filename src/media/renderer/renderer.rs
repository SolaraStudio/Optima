#[derive(Debug, Clone)]
pub struct MediaRenderer {
    pub width: u32,
    pub height: u32,
}

impl MediaRenderer {
    pub fn new(width: u32, height: u32) -> Self {
        MediaRenderer { width, height }
    }
    pub fn resize(&mut self, w: u32, h: u32) {
        self.width = w;
        self.height = h;
    }
}
