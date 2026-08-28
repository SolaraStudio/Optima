use ffmpeg_next as ffmpeg;
use std::path::Path;

pub struct Demuxer {
    format_context: ffmpeg::format::context::Input,
    video_stream_index: Option<usize>,
    audio_stream_index: Option<usize>,
}

impl Demuxer {
    pub fn new(path: &Path) -> Self {
        ffmpeg::init().unwrap();
        let format_context = ffmpeg::format::input(&path).unwrap();
        let mut video_stream_index = None;
        let mut audio_stream_index = None;
        for (i, stream) in format_context.streams().enumerate() {
            if stream.parameters().medium() == ffmpeg::media::Type::Video {
                video_stream_index = Some(i);
            } else if stream.parameters().medium() == ffmpeg::media::Type::Audio {
                audio_stream_index = Some(i);
            }
        }
        Self {
            format_context,
            video_stream_index,
            audio_stream_index,
        }
    }

    pub fn get_video_stream(&self) -> Option<usize> {
        self.video_stream_index
    }

    pub fn get_audio_stream(&self) -> Option<usize> {
        self.audio_stream_index
    }

    pub fn read_packet(&mut self) -> Option<ffmpeg::packet::Packet> {
        let packet = self.format_context.packets().next();
        if let Some(Ok(packet)) = packet {
            Some(packet)
        } else {
            None
        }
    }
}
