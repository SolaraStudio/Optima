#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PlaybackState {
    Idle,
    Loading,
    Playing,
    Paused,
    Stopped,
    Buffering,
    Seeking,
    Ended,
    Error,
}

impl PlaybackState {
    pub fn is_playing(&self) -> bool {
        matches!(self, PlaybackState::Playing)
    }

    pub fn is_paused(&self) -> bool {
        matches!(self, PlaybackState::Paused)
    }

    pub fn is_stopped(&self) -> bool {
        matches!(self, PlaybackState::Stopped)
    }

    pub fn is_loading(&self) -> bool {
        matches!(self, PlaybackState::Loading | PlaybackState::Buffering)
    }

    pub fn is_ended(&self) -> bool {
        matches!(self, PlaybackState::Ended)
    }

    pub fn is_error(&self) -> bool {
        matches!(self, PlaybackState::Error)
    }
}
