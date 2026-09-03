#[derive(Debug, Clone, PartialEq)]
pub enum PlaybackState {
    Stopped,
    Playing,
    Paused,
    Buffering,
    Error(String),
}

pub struct PlaybackController {
    pub state: PlaybackState,
    pub position: f64,
    pub duration: f64,
    pub rate: f32,
}

impl Default for PlaybackController {
    fn default() -> Self {
        PlaybackController {
            state: PlaybackState::Stopped,
            position: 0.0,
            duration: 0.0,
            rate: 1.0,
        }
    }
}

impl PlaybackController {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn play(&mut self) {
        self.state = PlaybackState::Playing;
    }
    pub fn pause(&mut self) {
        self.state = PlaybackState::Paused;
    }
    pub fn stop(&mut self) {
        self.state = PlaybackState::Stopped;
        self.position = 0.0;
    }
    pub fn seek(&mut self, pos: f64) {
        self.position = pos;
    }
    pub fn set_rate(&mut self, rate: f32) {
        self.rate = rate;
    }
    pub fn is_playing(&self) -> bool {
        self.state == PlaybackState::Playing
    }
    pub fn progress(&self) -> f32 {
        if self.duration > 0.0 {
            (self.position / self.duration) as f32
        } else {
            0.0
        }
    }
}
