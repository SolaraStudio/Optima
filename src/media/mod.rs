pub mod audio;
pub mod autoplay;
pub mod codec;
pub mod decoder;
pub mod demuxer;
pub mod encoder;
pub mod playback;
pub mod renderer;
pub mod stream;
pub mod subtitle;
pub mod sync;

pub use audio::AudioEngine;
pub use demuxer::Demuxer;
pub use decoder::Decoder;
pub use codec::Codec;
pub use autoplay::AutoplayPolicy;
pub use playback::PlaybackState;
pub use stream::MediaStream;
pub use sync::SyncController;
pub use renderer::MediaRenderer;

use std::time::Duration;

pub struct MediaPipeline {
    pub is_playing: bool,
    pub position: Duration,
    pub duration: Duration,
    pub has_video: bool,
    pub has_audio: bool,
    pub playback_state: PlaybackState,
    pub sync_controller: SyncController,
    pub audio_engine: AudioEngine,
    pub video_decoder: decoder::Decoder,
}

impl MediaPipeline {
    pub fn new() -> Self {
        Self {
            is_playing: false,
            position: Duration::from_secs(0),
            duration: Duration::from_secs(0),
            has_video: false,
            has_audio: false,
            playback_state: PlaybackState::Idle,
            sync_controller: SyncController::new(),
            audio_engine: AudioEngine::new(),
            video_decoder: decoder::Decoder::new(),
        }
    }

    pub fn load_url(&mut self, url: &str) -> Result<(), String> {
        // Try to load as video
        if let Ok(()) = self.video_decoder.open(url) {
            self.has_video = true;
            if let Some(duration) = self.video_decoder.get_duration() {
                self.duration = duration;
            }
        }
        // Try to load as audio (via demuxer)
        if let Ok(demuxer) = demuxer::Demuxer::new(url) {
            self.has_audio = true;
            if let Some(duration) = demuxer.get_duration() {
                self.duration = duration;
            }
            self.audio_engine.set_demuxer(demuxer);
        }
        if !self.has_video && !self.has_audio {
            return Err("No media tracks found".to_string());
        }
        Ok(())
    }

    pub fn play(&mut self) {
        self.is_playing = true;
        self.playback_state = PlaybackState::Playing;
        self.sync_controller.start();
        self.audio_engine.play();
    }

    pub fn pause(&mut self) {
        self.is_playing = false;
        self.playback_state = PlaybackState::Paused;
        self.sync_controller.pause();
        self.audio_engine.pause();
    }

    pub fn stop(&mut self) {
        self.is_playing = false;
        self.position = Duration::from_secs(0);
        self.playback_state = PlaybackState::Stopped;
        self.sync_controller.stop();
        self.audio_engine.stop();
    }

    pub fn seek(&mut self, pos: Duration) {
        self.position = pos;
        self.sync_controller.seek(pos);
        self.audio_engine.seek(pos);
        self.video_decoder.seek(pos);
    }

    pub fn set_duration(&mut self, dur: Duration) {
        self.duration = dur;
        self.sync_controller.set_duration(dur);
    }

    pub fn get_position(&self) -> Duration {
        if self.is_playing {
            self.sync_controller.get_current_time()
        } else {
            self.position
        }
    }

    pub fn is_at_end(&self) -> bool {
        self.position >= self.duration && self.duration > Duration::from_secs(0)
    }

    pub fn get_progress(&self) -> f32 {
        if self.duration > Duration::from_secs(0) {
            self.position.as_secs_f32() / self.duration.as_secs_f32()
        } else {
            0.0
        }
    }

    pub fn next_video_frame(&mut self) -> Option<VideoFrame> {
        if !self.is_playing || !self.has_video {
            return None;
        }
        self.video_decoder.decode_next_frame()
    }

    pub fn next_audio_samples(&mut self) -> Option<AudioFrame> {
        if !self.is_playing || !self.has_audio {
            return None;
        }
        self.audio_engine.next_samples()
    }
}

impl Default for MediaPipeline {
    fn default() -> Self {
        Self::new()
    }
}

pub struct VideoFrame {
    pub width: u32,
    pub height: u32,
    pub data: Vec<u8>,
    pub pts: u64,
    pub dts: u64,
    pub duration: u64,
    pub is_keyframe: bool,
}

pub struct AudioFrame {
    pub data: Vec<f32>,
    pub sample_rate: u32,
    pub channels: u8,
    pub pts: u64,
    pub duration: u64,
}
