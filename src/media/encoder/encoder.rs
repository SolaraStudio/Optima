use crate::media::codec::Codec;

pub struct Encoder {
    pub codec: Codec,
    pub bitrate: u32,
}

impl Encoder {
    pub fn new(codec: Codec, bitrate: u32) -> Self {
        Encoder { codec, bitrate }
    }
    pub fn set_bitrate(&mut self, bitrate: u32) {
        self.bitrate = bitrate;
    }
}
