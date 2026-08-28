#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Codec {
    H264,
    H265,
    VP9,
    AV1,
    AAC,
    MP3,
    Opus,
    Vorbis,
    FLAC,
    PCM,
    MPEG4,
    VP8,
    Theora,
}

impl Codec {
    pub fn from_mime_type(mime: &str) -> Option<Self> {
        match mime {
            "video/avc" => Some(Codec::H264),
            "video/hevc" => Some(Codec::H265),
            "video/vp9" => Some(Codec::VP9),
            "video/av01" => Some(Codec::AV1),
            "video/mp4v-es" => Some(Codec::MPEG4),
            "video/vp8" => Some(Codec::VP8),
            "video/theora" => Some(Codec::Theora),
            "audio/mp4a-latm" => Some(Codec::AAC),
            "audio/mpeg" => Some(Codec::MP3),
            "audio/opus" => Some(Codec::Opus),
            "audio/vorbis" => Some(Codec::Vorbis),
            "audio/flac" => Some(Codec::FLAC),
            "audio/pcm" => Some(Codec::PCM),
            _ => None,
        }
    }

    pub fn mime_type(&self) -> &'static str {
        match self {
            Codec::H264 => "video/avc",
            Codec::H265 => "video/hevc",
            Codec::VP9 => "video/vp9",
            Codec::AV1 => "video/av01",
            Codec::MPEG4 => "video/mp4v-es",
            Codec::VP8 => "video/vp8",
            Codec::Theora => "video/theora",
            Codec::AAC => "audio/mp4a-latm",
            Codec::MP3 => "audio/mpeg",
            Codec::Opus => "audio/opus",
            Codec::Vorbis => "audio/vorbis",
            Codec::FLAC => "audio/flac",
            Codec::PCM => "audio/pcm",
        }
    }

    pub fn is_video(&self) -> bool {
        matches!(self,
            Codec::H264 | Codec::H265 | Codec::VP9 | Codec::AV1 |
            Codec::MPEG4 | Codec::VP8 | Codec::Theora
        )
    }

    pub fn is_audio(&self) -> bool {
        !self.is_video()
    }

    pub fn is_lossless(&self) -> bool {
        matches!(self, Codec::FLAC | Codec::PCM)
    }

    pub fn supports_hardware_decoding(&self) -> bool {
        matches!(self,
            Codec::H264 | Codec::H265 | Codec::VP9 | Codec::AV1 |
            Codec::AAC | Codec::MP3 | Codec::Opus
        )
    }

    pub fn get_ffmpeg_codec(&self) -> &'static str {
        match self {
            Codec::H264 => "h264",
            Codec::H265 => "hevc",
            Codec::VP9 => "vp9",
            Codec::AV1 => "av1",
            Codec::MPEG4 => "mpeg4",
            Codec::VP8 => "vp8",
            Codec::Theora => "theora",
            Codec::AAC => "aac",
            Codec::MP3 => "mp3",
            Codec::Opus => "opus",
            Codec::Vorbis => "vorbis",
            Codec::FLAC => "flac",
            Codec::PCM => "pcm",
        }
    }
}
