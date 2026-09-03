#[derive(Default)]
pub struct AvSync {
    pub audio_offset_ms: i64,
    pub video_offset_ms: i64,
}


impl AvSync {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn sync(&self, audio_pts: f64, video_pts: f64) -> f64 {
        (audio_pts + self.audio_offset_ms as f64) - (video_pts + self.video_offset_ms as f64)
    }
}
