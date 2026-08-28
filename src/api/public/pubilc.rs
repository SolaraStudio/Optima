use crate::render::VelloRenderer;
use crate::media::MediaPipeline;
use crate::media::audio::AudioEngine;
use crate::dom::Document;
use crate::css::ComputedStyle;
use crate::layout::block::BlockLayout;
use crate::settings::Settings;

pub struct OptimaAPI {
    pub renderer: VelloRenderer,
    pub media: MediaPipeline,
    pub audio: AudioEngine,
    pub settings: Settings,
}

impl OptimaAPI {
    pub fn new(surface: jni::objects::JObject) -> Self {
        let renderer = VelloRenderer::new(surface);
        let media = MediaPipeline::new();
        let audio = AudioEngine::new();
        let settings = Settings::default();
        Self {
            renderer,
            media,
            audio,
            settings,
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

    pub fn get_settings(&self) -> &Settings {
        &self.settings
    }

    pub fn update_settings(&mut self, settings: Settings) {
        self.settings = settings;
    }

    pub fn get_media_position(&self) -> std::time::Duration {
        self.media.get_position()
    }

    pub fn get_media_duration(&self) -> std::time::Duration {
        self.media.duration
    }

    pub fn is_media_playing(&self) -> bool {
        self.media.is_playing
    }

    pub fn get_media_progress(&self) -> f32 {
        self.media.get_progress()
    }

    pub fn load_html(&mut self, html: &str) {
        // Load HTML and render
    }

    pub fn load_css(&mut self, css: &str) {
        // Load CSS and apply styles
    }

    pub fn set_window_size(&mut self, width: u32, height: u32) {
        self.renderer.resize(width, height);
    }
}
