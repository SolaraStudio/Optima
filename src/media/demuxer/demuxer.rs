pub struct Demuxer {
    pub format: String,
}

impl Demuxer {
    pub fn new(format: &str) -> Self { Demuxer { format: format.to_string() } }

    pub fn detect_format(data: &[u8]) -> &str {
        if data.len() < 4 { return "unknown"; }
        match &data[0..4] {
            b"\x1a\x45\xdf\xa3" => "webm",
            b"\x00\x00\x00\x1c" | b"\x00\x00\x00\x20" => "mp4",
            b"RIFF" => "avi",
            b"ID3" | [0xff, 0xfb, ..] => "mp3",
            b"fLaC" => "flac",
            _ => "unknown",
        }
    }
}
