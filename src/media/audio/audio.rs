use super::demuxer::Demuxer;
use super::AudioFrame;
use symphonia::core::codecs::DecoderOptions;
use symphonia::core::formats::FormatOptions;
use symphonia::core::meta::MetadataOptions;
use symphonia::core::probe::Hint;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{Stream, StreamConfig};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use anyhow::Result;

pub struct AudioEngine {
    stream: Option<Stream>,
    sample_rate: u32,
    channels: u8,
    is_playing: bool,
    volume: f32,
    buffer: Arc<Mutex<Vec<f32>>>,
    demuxer: Option<Demuxer>,
    decoder: Option<symphonia::core::codecs::Decoder>,
    current_pts: u64,
}

impl AudioEngine {
    pub fn new() -> Self {
        Self {
            stream: None,
            sample_rate: 44100,
            channels: 2,
            is_playing: false,
            volume: 1.0,
            buffer: Arc::new(Mutex::new(Vec::new())),
            demuxer: None,
            decoder: None,
            current_pts: 0,
        }
    }

    pub fn set_demuxer(&mut self, demuxer: Demuxer) {
        let mut demuxer = demuxer;
        if let Some(audio_stream) = demuxer.get_audio_stream() {
            if let Some(track) = demuxer.get_audio_track_info(audio_stream) {
                self.sample_rate = track.sample_rate;
                self.channels = track.channels as u8;
                // Create Symphonia decoder
                let hint = Hint::new();
                let format_opts = FormatOptions::default();
                let meta_opts = MetadataOptions::default();
                let mss = symphonia::core::io::MediaSourceStream::new(
                    Box::new(std::io::Cursor::new(Vec::new())),
                    Default::default(),
                );
                // We need to pass the actual data; this is a placeholder
                // In practice, we'd decode from the demuxer's packets
                let dec = symphonia::default::get_codecs()
                    .make(&track.codec_params, &DecoderOptions::default())
                    .ok();
                self.decoder = dec;
            }
        }
        self.demuxer = Some(demuxer);
    }

    pub fn play(&mut self) {
        if self.stream.is_none() {
            self.build_stream();
        }
        if let Some(stream) = &self.stream {
            stream.play().unwrap();
        }
        self.is_playing = true;
    }

    pub fn pause(&mut self) {
        if let Some(stream) = &self.stream {
            stream.pause().unwrap();
        }
        self.is_playing = false;
    }

    pub fn stop(&mut self) {
        if let Some(stream) = self.stream.take() {
            drop(stream);
        }
        self.is_playing = false;
        self.current_pts = 0;
    }

    pub fn seek(&mut self, position: Duration) {
        if let Some(demuxer) = &mut self.demuxer {
            demuxer.seek(position);
        }
        self.current_pts = position.as_micros() as u64;
    }

    pub fn next_samples(&mut self) -> Option<AudioFrame> {
        if !self.is_playing {
            return None;
        }
        // Decode next packet from demuxer
        if let Some(demuxer) = &mut self.demuxer {
            if let Some(packet) = demuxer.read_packet() {
                // Decode with Symphonia
                if let Some(decoder) = &mut self.decoder {
                    if let Ok(decoded) = decoder.decode(&packet) {
                        if let Some(audio_buf) = decoded.audio() {
                            let samples = audio_buf.samples();
                            let planes = audio_buf.planes();
                            let mut data = Vec::with_capacity(samples * planes.count());
                            for i in 0..samples {
                                for ch in 0..planes.count() {
                                    let sample = planes.plane(ch).get_f32(i);
                                    data.push(sample);
                                }
                            }
                            return Some(AudioFrame {
                                data,
                                sample_rate: self.sample_rate,
                                channels: self.channels,
                                pts: packet.pts,
                                duration: packet.duration,
                            });
                        }
                    }
                }
            }
        }
        None
    }

    pub fn set_volume(&mut self, volume: f32) {
        self.volume = volume.clamp(0.0, 1.0);
    }

    pub fn is_playing(&self) -> bool {
        self.is_playing
    }

    fn build_stream(&mut self) {
        let host = cpal::default_host();
        let device = host.default_output_device().expect("No audio output device");
        let config = device.default_output_config().expect("No default output config");
        let sample_rate = config.sample_rate().0;
        let channels = self.channels as usize;

        let buffer = self.buffer.clone();
        let stream = device
            .build_output_stream(
                &config.into(),
                move |output: &mut [f32], _| {
                    let mut buf = buffer.lock().unwrap();
                    let len = output.len().min(buf.len());
                    output[..len].copy_from_slice(&buf[..len]);
                    buf.drain(..len);
                },
                |err| eprintln!("Audio error: {}", err),
                None,
            )
            .unwrap();
        self.stream = Some(stream);
    }
}

impl Default for AudioEngine {
    fn default() -> Self {
        Self::new()
    }
}
