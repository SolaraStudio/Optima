use crate::render::VelloRenderer;
use crate::media::MediaPipeline;
use crate::media::audio::AudioEngine;

pub struct OptimaAPI {
    pub renderer: VelloRenderer,
    pub media: MediaPipeline,
    pub audio: AudioEngine,
}

impl OptimaAPI {
    pub fn new(surface: jni::objects::JObject) -> Self {
        let renderer = VelloRenderer::new(surface);
        let media = MediaPipeline::new();
        let audio = AudioEngine::new();
        Self {
            renderer,
            media,
            audio,
        }
    }

    pub fn version() -> &'static str {
        "0.150.10"
    }

    pub fn render(&mut self) {
        self.renderer.render();
    }

    pub fn play_audio(&mut self, freq: f32, duration: f32) {
        self.audio.play_tone(freq, duration);
    }

    pub fn play_media(&mut self) {
        self.media.play();
    }

    pub fn pause_media(&mut self) {
        self.media.pause();
    }

    pub fn seek_media(&mut self, pos: std::time::Duration) {
        self.media.seek(pos);
    }
}
