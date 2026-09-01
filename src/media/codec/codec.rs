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
}

impl Codec {
    pub fn from_mime_type(mime: &str) -> Option<Self> {
        match mime {
            "video/avc" => Some(Codec::H264),
            "video/hevc" => Some(Codec::H265),
            "video/vp9" => Some(Codec::VP9),
            "video/av01" => Some(Codec::AV1),
            "audio/mp4a-latm" => Some(Codec::AAC),
            "audio/mpeg" => Some(Codec::MP3),
            "audio/opus" => Some(Codec::Opus),
            "audio/vorbis" => Some(Codec::Vorbis),
            "audio/flac" => Some(Codec::FLAC),
            _ => None,
        }
    }

    pub fn mime_type(&self) -> &'static str {
        match self {
            Codec::H264 => "video/avc",
            Codec::H265 => "video/hevc",
            Codec::VP9 => "video/vp9",
            Codec::AV1 => "video/av01",
            Codec::AAC => "audio/mp4a-latm",
            Codec::MP3 => "audio/mpeg",
            Codec::Opus => "audio/opus",
            Codec::Vorbis => "audio/vorbis",
            Codec::FLAC => "audio/flac",
            Codec::PCM => "audio/pcm",
        }
    }

    pub fn is_video(&self) -> bool {
        matches!(self, Codec::H264 | Codec::H265 | Codec::VP9 | Codec::AV1)
    }

    pub fn is_audio(&self) -> bool {
        !self.is_video()
    }
}
