use crate::media::volume::VolumeControl;

#[derive(Debug, Clone)]
pub struct AudioTrack {
    pub id: u32,
    pub language: String,
    pub channels: u32,
    pub sample_rate: u32,
    pub volume: VolumeControl,
}

impl AudioTrack {
    pub fn new(id: u32) -> Self {
        AudioTrack {
            id,
            language: "en".to_string(),
            channels: 2,
            sample_rate: 44100,
            volume: VolumeControl::new(),
        }
    }
}
