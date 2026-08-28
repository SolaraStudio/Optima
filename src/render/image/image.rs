use std::io::Cursor;

pub struct Image {
    pub width: u32,
    pub height: u32,
    pub data: Vec<u8>,
    pub format: ImageFormat,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ImageFormat {
    PNG,
    JPEG,
    WebP,
    GIF,
    BMP,
    Unknown,
}

impl Image {
    pub fn from_bytes(data: &[u8]) -> Option<Self> {
        let format = Self::detect_format(data);
        match format {
            ImageFormat::PNG => Self::decode_png(data),
            ImageFormat::JPEG => Self::decode_jpeg(data),
            _ => None,
        }
    }

    pub fn detect_format(data: &[u8]) -> ImageFormat {
        if data.len() < 8 {
            return ImageFormat::Unknown;
        }
        if data[0..4] == [0x89, 0x50, 0x4E, 0x47] {
            return ImageFormat::PNG;
        }
        if data[0..2] == [0xFF, 0xD8] {
            return ImageFormat::JPEG;
        }
        if data[0..4] == [0x52, 0x49, 0x46, 0x46] && data[8..12] == [0x57, 0x45, 0x42, 0x50] {
            return ImageFormat::WebP;
        }
        if data[0..4] == [0x47, 0x49, 0x46, 0x38] {
            return ImageFormat::GIF;
        }
        if data[0..2] == [0x42, 0x4D] {
            return ImageFormat::BMP;
        }
        ImageFormat::Unknown
    }

    fn decode_png(data: &[u8]) -> Option<Self> {
        let decoder = png::Decoder::new(Cursor::new(data));
        if let Ok(mut reader) = decoder.read_info() {
            let info = reader.info();
            let mut buf = vec![0; info.buffer_size()];
            if let Ok(_) = reader.next_frame(&mut buf) {
                return Some(Image {
                    width: info.width,
                    height: info.height,
                    data: buf,
                    format: ImageFormat::PNG,
                });
            }
        }
        None
    }

    fn decode_jpeg(data: &[u8]) -> Option<Self> {
        let decoder = jpeg_decoder::Decoder::new(Cursor::new(data));
        if let Ok(metadata) = decoder.info() {
            let mut buf = Vec::new();
            if let Ok(decoded) = decoder.decode(&mut buf) {
                return Some(Image {
                    width: metadata.width as u32,
                    height: metadata.height as u32,
                    data: buf,
                    format: ImageFormat::JPEG,
                });
            }
        }
        None
    }

    pub fn get_rgba(&self) -> Vec<u8> {
        self.data.clone()
    }

    pub fn resize(&self, _new_width: u32, _new_height: u32) -> Self {
        // Simple nearest-neighbor resize placeholder
        Image {
            width: _new_width,
            height: _new_height,
            data: self.data.clone(),
            format: self.format,
        }
    }
}
