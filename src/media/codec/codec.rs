#[derive(Debug, Clone, PartialEq)]
pub enum Codec {
    H264,
    H265,
    VP8,
    VP9,
    AV1,
    AAC,
    MP3,
    Opus,
    Vorbis,
    FLAC,
    PCM,
    Unknown(String),
}

impl Codec {
    pub fn from_mime(mime: &str) -> Self {
        match mime {
            "video/h264" | "avc1" => Codec::H264,
            "video/h265" => Codec::H265,
            "video/vp8" => Codec::VP8,
            "video/vp9" => Codec::VP9,
            "video/av1" => Codec::AV1,
            "audio/aac" => Codec::AAC,
            "audio/mpeg" | "audio/mp3" => Codec::MP3,
            "audio/opus" => Codec::Opus,
            "audio/vorbis" => Codec::Vorbis,
            "audio/flac" => Codec::FLAC,
            _ => Codec::Unknown(mime.to_string()),
        }
    }

    pub fn is_video(&self) -> bool {
        matches!(self, Codec::H264 | Codec::H265 | Codec::VP8 | Codec::VP9 | Codec::AV1)
    }

    pub fn is_audio(&self) -> bool {
        matches!(self, Codec::AAC | Codec::MP3 | Codec::Opus | Codec::Vorbis | Codec::FLAC | Codec::PCM)
    }
}
