use super::VideoFrame;
use yscv_video::decoder::{Decoder as YscvDecoder, CodecType};
use yscv_video::frame::Frame;
use std::time::Duration;

pub struct Decoder {
    inner: Option<YscvDecoder>,
    video_track_index: Option<usize>,
    width: u32,
    height: u32,
    current_pts: Duration,
    codec_type: CodecType,
}

impl Decoder {
    pub fn new() -> Self {
        Self {
            inner: None,
            video_track_index: None,
            width: 0,
            height: 0,
            current_pts: Duration::from_secs(0),
            codec_type: CodecType::H264,
        }
    }

    pub fn open(&mut self, path: &str) -> Result<(), String> {
        let demuxer = yscv_video::demuxer::Demuxer::new(path)
            .map_err(|e| format!("Failed to open demuxer: {:?}", e))?;

        let video_track = demuxer.video_tracks().next()
            .ok_or_else(|| "No video track found".to_string())?;
        let track_index = video_track.index;

        // Determine codec type
        let codec_type = match video_track.codec_name.as_deref() {
            Some(name) if name.contains("h264") || name.contains("avc") => CodecType::H264,
            Some(name) if name.contains("hevc") || name.contains("h265") => CodecType::HEVC,
            Some(name) if name.contains("av1") => CodecType::AV1,
            _ => CodecType::H264,
        };

        let decoder = YscvDecoder::new(codec_type)
            .map_err(|e| format!("Failed to create decoder: {:?}", e))?;

        self.inner = Some(decoder);
        self.video_track_index = Some(track_index);
        self.codec_type = codec_type;

        // Get resolution
        if let Some(stream) = demuxer.streams().iter().find(|s| s.index == track_index) {
            self.width = stream.codec_params.width.unwrap_or(0);
            self.height = stream.codec_params.height.unwrap_or(0);
        }

        Ok(())
    }

    pub fn decode_next_frame(&mut self) -> Option<VideoFrame> {
        let decoder = self.inner.as_mut()?;
        let track_index = self.video_track_index?;

        // We need a demuxer; this is a simplified version
        // In practice, we'd use the demuxer from the media pipeline
        // For now, we'll return None (integration is handled in MediaPipeline)
        None
    }

    pub fn decode_packet(&mut self, data: &[u8], pts: u64, dts: u64, duration: u64) -> Option<VideoFrame> {
        let decoder = self.inner.as_mut()?;
        if let Ok(Some(frame)) = decoder.decode(data) {
            let rgba = frame.rgba();
            return Some(VideoFrame {
                width: self.width,
                height: self.height,
                data: rgba,
                pts,
                dts,
                duration,
                is_keyframe: true,
            });
        }
        None
    }

    pub fn get_width(&self) -> u32 {
        self.width
    }

    pub fn get_height(&self) -> u32 {
        self.height
    }

    pub fn get_duration(&self) -> Option<Duration> {
        None // Implemented in demuxer
    }

    pub fn seek(&mut self, _position: Duration) {
        // Placeholder – seek in demuxer
    }
}

impl Default for Decoder {
    fn default() -> Self {
        Self::new()
    }
}
