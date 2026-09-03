#[derive(Debug, Clone)]
pub struct VideoTrack {
    pub id: u32,
    pub width: u32,
    pub height: u32,
    pub fps: f32,
    pub codec: String,
}

impl VideoTrack {
    pub fn new(id: u32) -> Self {
        VideoTrack { id, width: 1920, height: 1080, fps: 30.0, codec: "h264".to_string() }
    }

    pub fn with_resolution(mut self, w: u32, h: u32) -> Self { self.width = w; self.height = h; self }
    pub fn with_fps(mut self, fps: f32) -> Self { self.fps = fps; self }
}
