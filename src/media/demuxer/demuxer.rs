use yscv_video::demuxer::Demuxer as YscvDemuxer;
use std::path::Path;
use std::time::Duration;
use anyhow::Result;

pub struct Demuxer {
    inner: Option<YscvDemuxer>,
    audio_track_index: Option<usize>,
    video_track_index: Option<usize>,
    duration: Duration,
}

pub struct AudioTrackInfo {
    pub sample_rate: u32,
    pub channels: u16,
    pub codec_name: String,
    pub bitrate: u32,
}

impl Demuxer {
    pub fn new(path: &str) -> Result<Self> {
        let inner = YscvDemuxer::new(path).ok();
        let mut demuxer = Self {
            inner,
            audio_track_index: None,
            video_track_index: None,
            duration: Duration::from_secs(0),
        };
        if let Some(ref d) = demuxer.inner {
            demuxer.duration = d.duration().unwrap_or(Duration::from_secs(0));
            // Find tracks
            for stream in d.streams() {
                if stream.codec_type == yscv_video::demuxer::StreamType::Audio {
                    demuxer.audio_track_index = Some(stream.index);
                } else if stream.codec_type == yscv_video::demuxer::StreamType::Video {
                    demuxer.video_track_index = Some(stream.index);
                }
            }
        }
        Ok(demuxer)
    }

    pub fn get_audio_stream(&self) -> Option<usize> {
        self.audio_track_index
    }

    pub fn get_video_stream(&self) -> Option<usize> {
        self.video_track_index
    }

    pub fn get_audio_track_info(&self, index: usize) -> Option<AudioTrackInfo> {
        if let Some(ref d) = self.inner {
            for stream in d.streams() {
                if stream.index == index {
                    return Some(AudioTrackInfo {
                        sample_rate: stream.codec_params.sample_rate.unwrap_or(44100),
                        channels: stream.codec_params.channels.unwrap_or(2) as u16,
                        codec_name: stream.codec_name.clone().unwrap_or("unknown".to_string()),
                        bitrate: stream.codec_params.bitrate.unwrap_or(0) as u32,
                    });
                }
            }
        }
        None
    }

    pub fn get_duration(&self) -> Option<Duration> {
        if self.duration > Duration::from_secs(0) {
            Some(self.duration)
        } else {
            None
        }
    }

    pub fn read_packet(&mut self) -> Option<yscv_video::demuxer::Packet> {
        if let Some(ref mut d) = self.inner {
            // yscv-video doesn't have a direct packet iterator; we'll use the packet iterator
            for packet in d.packets() {
                return Some(packet);
            }
        }
        None
    }

    pub fn seek(&mut self, position: Duration) {
        if let Some(ref mut d) = self.inner {
            // yscv-video seek not implemented; placeholder
        }
    }
}
