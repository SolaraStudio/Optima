#[derive(Debug, Clone)]
pub struct ImageData {
    pub pixels: Vec<u8>,
    pub width: u32,
    pub height: u32,
    pub format: ImageFormat,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ImageFormat {
    RGBA8,
    RGB8,
    BGRA8,
    BGR8,
}

impl ImageData {
    pub fn new(width: u32, height: u32, format: ImageFormat) -> Self {
        let bpp = match format {
            ImageFormat::RGBA8 | ImageFormat::BGRA8 => 4,
            _ => 3,
        };
        ImageData {
            pixels: vec![0; (width * height * bpp) as usize],
            width,
            height,
            format,
        }
    }

    pub fn from_rgba(width: u32, height: u32, data: Vec<u8>) -> Self {
        ImageData {
            pixels: data,
            width,
            height,
            format: ImageFormat::RGBA8,
        }
    }

    pub fn fill(&mut self, r: u8, g: u8, b: u8, a: u8) {
        for chunk in self.pixels.as_chunks_mut::<4>().0 {
            chunk[0] = r;
            chunk[1] = g;
            chunk[2] = b;
            chunk[3] = a;
        }
    }

    pub fn get_pixel(&self, x: u32, y: u32) -> Option<(u8, u8, u8, u8)> {
        if x >= self.width || y >= self.height {
            return None;
        }
        let idx = ((y * self.width + x) * 4) as usize;
        if idx + 4 > self.pixels.len() {
            return None;
        }
        Some((
            self.pixels[idx],
            self.pixels[idx + 1],
            self.pixels[idx + 2],
            self.pixels[idx + 3],
        ))
    }

    pub fn set_pixel(&mut self, x: u32, y: u32, r: u8, g: u8, b: u8, a: u8) {
        if x >= self.width || y >= self.height {
            return;
        }
        let idx = ((y * self.width + x) * 4) as usize;
        if idx + 4 <= self.pixels.len() {
            self.pixels[idx] = r;
            self.pixels[idx + 1] = g;
            self.pixels[idx + 2] = b;
            self.pixels[idx + 3] = a;
        }
    }

    pub fn bytes(&self) -> &[u8] {
        &self.pixels
    }
}
