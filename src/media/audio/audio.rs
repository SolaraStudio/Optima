use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait},
    Stream, StreamConfig,
};
use std::sync::{Arc, Mutex};

pub struct AudioEngine {
    stream: Option<Stream>,
    sample_rate: u32,
    volume: f32,
    is_muted: bool,
}

impl AudioEngine {
    pub fn new() -> Self {
        Self {
            stream: None,
            sample_rate: 44100,
            volume: 1.0,
            is_muted: false,
        }
    }

    pub fn play_tone(&mut self, frequency: f32, duration: f32) {
        let host = cpal::default_host();
        let device = host.default_output_device().expect("No audio output device");
        let config = device.default_output_config().expect("No default config");
        let sample_rate = config.sample_rate().0;

        let num_samples = (duration * sample_rate as f32) as usize;
        let mut data = Vec::with_capacity(num_samples);
        for i in 0..num_samples {
            let t = i as f32 / sample_rate as f32;
            let sample = (2.0 * std::f32::consts::PI * frequency * t).sin();
            data.push(sample);
        }

        let stream = device
            .build_output_stream(
                &config.into(),
                move |output: &mut [f32], _| {
                    let len = output.len().min(data.len());
                    output[..len].copy_from_slice(&data[..len]);
                },
                |err| eprintln!("Audio error: {}", err),
                None,
            )
            .expect("Failed to build audio stream");

        stream.play().expect("Failed to play audio");
        self.stream = Some(stream);
    }

    pub fn set_volume(&mut self, volume: f32) {
        self.volume = volume.clamp(0.0, 1.0);
    }

    pub fn set_muted(&mut self, muted: bool) {
        self.is_muted = muted;
    }

    pub fn get_volume(&self) -> f32 {
        self.volume
    }

    pub fn is_muted(&self) -> bool {
        self.is_muted
    }

    pub fn stop(&mut self) {
        if let Some(stream) = self.stream.take() {
            drop(stream);
        }
    }
}

impl Default for AudioEngine {
    fn default() -> Self {
        Self::new()
    }
}
