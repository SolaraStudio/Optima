use crate::media::codec::Codec;

pub struct Decoder {
    pub codec: Codec,
    pub initialized: bool,
}

impl Decoder {
    pub fn new(codec: Codec) -> Self {
        Decoder {
            codec,
            initialized: false,
        }
    }
    pub fn initialize(&mut self) -> Result<(), String> {
        self.initialized = true;
        Ok(())
    }
    pub fn is_initialized(&self) -> bool {
        self.initialized
    }
}
