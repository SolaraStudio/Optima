pub struct Decoder {
    // MediaCodec JNI bridge for video; ffmpeg for audio
}

impl Decoder {
    pub fn new() -> Self {
        Self {}
    }

    pub fn decode_video(&self, _data: &[u8]) -> Option<crate::media::VideoFrame> {
        None
    }

    pub fn decode_audio(&self, _data: &[u8]) -> Option<crate::media::AudioFrame> {
        None
    }
}
