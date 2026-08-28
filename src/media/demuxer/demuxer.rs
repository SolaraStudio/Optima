use ffmpeg_next as ffmpeg;
use std::path::Path;
use std::time::Duration;

pub struct Demuxer {
    format_context: ffmpeg::format::context::Input,
    video_stream_index: Option<usize>,
    audio_stream_index: Option<usize>,
    duration: Duration,
    bit_rate: u64,
}

impl Demuxer {
    pub fn new(path: &Path) -> Result<Self, String> {
        ffmpeg::init().map_err(|e| format!("FFmpeg init failed: {}", e))?;
        let format_context = ffmpeg::format::input(path)
            .map_err(|e| format!("Failed to open input: {}", e))?;

        let mut video_stream_index = None;
        let mut audio_stream_index = None;
        let duration = Duration::from_micros(
            format_context.duration() as u64
        );
        let bit_rate = format_context.bit_rate();

        for (i, stream) in format_context.streams().enumerate() {
            if stream.parameters().medium() == ffmpeg::media::Type::Video {
                video_stream_index = Some(i);
            } else if stream.parameters().medium() == ffmpeg::media::Type::Audio {
                audio_stream_index = Some(i);
            }
        }

        Ok(Self {
            format_context,
            video_stream_index,
            audio_stream_index,
            duration,
            bit_rate,
        })
    }

    pub fn get_video_stream(&self) -> Option<usize> {
        self.video_stream_index
    }

    pub fn get_audio_stream(&self) -> Option<usize> {
        self.audio_stream_index
    }

    pub fn get_duration(&self) -> Duration {
        self.duration
    }

    pub fn get_bit_rate(&self) -> u64 {
        self.bit_rate
    }

    pub fn read_packet(&mut self) -> Option<ffmpeg::packet::Packet> {
        if let Some(Ok(packet)) = self.format_context.packets().next() {
            Some(packet)
        } else {
            None
        }
    }

    pub fn seek(&mut self, timestamp: Duration) -> bool {
        let timestamp_us = timestamp.as_micros() as i64;
        self.format_context.seek(timestamp_us, ..).is_ok()
    }

    pub fn get_video_codec_name(&self) -> Option<String> {
        if let Some(idx) = self.video_stream_index {
            if let Some(stream) = self.format_context.streams().nth(idx) {
                let codec = stream.parameters().codec();
                return Some(codec.name().to_string());
            }
        }
        None
    }

    pub fn get_audio_codec_name(&self) -> Option<String> {
        if let Some(idx) = self.audio_stream_index {
            if let Some(stream) = self.format_context.streams().nth(idx) {
                let codec = stream.parameters().codec();
                return Some(codec.name().to_string());
            }
        }
        None
    }

    pub fn get_video_resolution(&self) -> Option<(u32, u32)> {
        if let Some(idx) = self.video_stream_index {
            if let Some(stream) = self.format_context.streams().nth(idx) {
                let params = stream.parameters();
                if let Ok(ffmpeg::codec::Parameters::Video(video)) = params.try_into() {
                    return Some((video.width(), video.height()));
                }
            }
        }
        None
    }

    pub fn get_audio_sample_rate(&self) -> Option<u32> {
        if let Some(idx) = self.audio_stream_index {
            if let Some(stream) = self.format_context.streams().nth(idx) {
                let params = stream.parameters();
                if let Ok(ffmpeg::codec::Parameters::Audio(audio)) = params.try_into() {
                    return Some(audio.rate());
                }
            }
        }
        None
    }
}
