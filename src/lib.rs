//! Optima – Custom WebView Engine for Solara Browser

pub mod jni;
pub mod render;
pub mod media;
pub mod dom;
pub mod css;
pub mod layout;
pub mod net;
pub mod events;
pub mod platform;
pub mod api;
pub mod config;
pub mod text;
pub mod utils;
pub mod android;

// Re-export commonly used types
pub use render::VelloRenderer;
pub use media::MediaPipeline;
pub use media::audio::AudioEngine;
