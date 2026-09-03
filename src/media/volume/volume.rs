#[derive(Debug, Clone)]
pub struct VolumeControl {
    pub volume: f32,
    pub muted: bool,
}

impl Default for VolumeControl {
    fn default() -> Self { VolumeControl { volume: 1.0, muted: false } }
}

impl VolumeControl {
    pub fn new() -> Self { Self::default() }
    pub fn set_volume(&mut self, v: f32) { self.volume = v.clamp(0.0, 1.0); }
    pub fn mute(&mut self) { self.muted = true; }
    pub fn unmute(&mut self) { self.muted = false; }
    pub fn toggle_mute(&mut self) { self.muted = !self.muted; }
    pub fn effective_volume(&self) -> f32 { if self.muted { 0.0 } else { self.volume } }
}
