pub mod audio;
pub mod demuxer;
pub mod decoder;
pub mod codec;
pub mod autoplay;

use std::time::Duration;

pub struct MediaPipeline {
    pub is_playing: bool,
    pub position: Duration,
}

impl MediaPipeline {
    pub fn new() -> Self {
        Self {
            is_playing: false,
            position: Duration::from_secs(0),
        }
    }

    pub fn play(&mut self) {
        self.is_playing = true;
    }

    pub fn pause(&mut self) {
        self.is_playing = false;
    }

    pub fn seek(&mut self, pos: Duration) {
        self.position = pos;
    }
}

pub struct VideoFrame {
    pub width: u32,
    pub height: u32,
    pub data: Vec<u8>,
    pub pts: u64,
}

pub struct AudioFrame {
    pub data: Vec<f32>,
    pub sample_rate: u32,
    pub channels: u8,
    pub pts: u64,
}
