use std::io::Cursor;
use png;
use jpeg_decoder;
use image::{ImageDecoder, DynamicImage, GenericImageView};
use webp;
use gif;

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
            ImageFormat::WebP => Self::decode_webp(data),
            ImageFormat::GIF => Self::decode_gif(data),
            ImageFormat::BMP => Self::decode_bmp(data),
            ImageFormat::Unknown => None,
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
                let mut rgba = Vec::with_capacity((info.width * info.height * 4) as usize);
                match info.color_type {
                    png::ColorType::Rgb => {
                        for chunk in buf.chunks(3) {
                            rgba.extend_from_slice(chunk);
                            rgba.push(255);
                        }
                    }
                    png::ColorType::Rgba => {
                        rgba = buf;
                    }
                    png::ColorType::Grayscale => {
                        for &p in &buf {
                            rgba.extend_from_slice(&[p, p, p, 255]);
                        }
                    }
                    png::ColorType::GrayscaleAlpha => {
                        for chunk in buf.chunks(2) {
                            rgba.extend_from_slice(&[chunk[0], chunk[0], chunk[0], chunk[1]]);
                        }
                    }
                    _ => return None,
                }
                return Some(Image {
                    width: info.width,
                    height: info.height,
                    data: rgba,
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
                let (width, height) = (metadata.width as u32, metadata.height as u32);
                let mut rgba = Vec::with_capacity((width * height * 4) as usize);
                match metadata.pixel_format {
                    jpeg_decoder::PixelFormat::RGB24 => {
                        for chunk in decoded.chunks(3) {
                            rgba.extend_from_slice(chunk);
                            rgba.push(255);
                        }
                    }
                    jpeg_decoder::PixelFormat::CMYK32 => {
                        for chunk in decoded.chunks(4) {
                            let c = chunk[0] as f32 / 255.0;
                            let m = chunk[1] as f32 / 255.0;
                            let y = chunk[2] as f32 / 255.0;
                            let k = chunk[3] as f32 / 255.0;
                            let r = ((1.0 - c) * (1.0 - k) * 255.0) as u8;
                            let g = ((1.0 - m) * (1.0 - k) * 255.0) as u8;
                            let b = ((1.0 - y) * (1.0 - k) * 255.0) as u8;
                            rgba.extend_from_slice(&[r, g, b, 255]);
                        }
                    }
                    _ => return None,
                }
                return Some(Image {
                    width,
                    height,
                    data: rgba,
                    format: ImageFormat::JPEG,
                });
            }
        }
        None
    }

    fn decode_webp(data: &[u8]) -> Option<Self> {
        let decoder = webp::Decoder::new(data);
        if let Some(decoded) = decoder.decode() {
            let rgba = decoded.to_rgba8();
            return Some(Image {
                width: rgba.width(),
                height: rgba.height(),
                data: rgba.into_vec(),
                format: ImageFormat::WebP,
            });
        }
        None
    }

    fn decode_gif(data: &[u8]) -> Option<Self> {
        let mut decoder = gif::DecodeOptions::new();
        decoder.set_color_output(gif::ColorOutput::RGBA);
        let mut reader = decoder.read_info(Cursor::new(data)).ok()?;
        let mut frame = reader.read_next_frame().ok()??;
        let rgba = frame.buffer.to_vec();
        let (width, height) = (reader.width() as u32, reader.height() as u32);
        Some(Image {
            width,
            height,
            data: rgba,
            format: ImageFormat::GIF,
        })
    }

    fn decode_bmp(data: &[u8]) -> Option<Self> {
        let decoder = image::io::Reader::new(Cursor::new(data))
            .with_guessed_format()
            .ok()?
            .decode()
            .ok()?;
        let rgba = decoder.to_rgba8();
        Some(Image {
            width: rgba.width(),
            height: rgba.height(),
            data: rgba.into_vec(),
            format: ImageFormat::BMP,
        })
    }

    pub fn get_rgba(&self) -> Vec<u8> {
        self.data.clone()
    }

    pub fn resize(&self, new_width: u32, new_height: u32) -> Self {
        if self.width == new_width && self.height == new_height {
            return Image {
                width: new_width,
                height: new_height,
                data: self.data.clone(),
                format: self.format,
            };
        }
        let old_width = self.width;
        let old_height = self.height;
        let mut new_data = vec![0u8; (new_width * new_height * 4) as usize];
        let x_ratio = old_width as f32 / new_width as f32;
        let y_ratio = old_height as f32 / new_height as f32;
        for y in 0..new_height {
            for x in 0..new_width {
                let px = (x as f32 * x_ratio) as u32;
                let py = (y as f32 * y_ratio) as u32;
                let src_idx = ((py * old_width + px) * 4) as usize;
                let dst_idx = ((y * new_width + x) * 4) as usize;
                new_data[dst_idx..dst_idx+4].copy_from_slice(&self.data[src_idx..src_idx+4]);
            }
        }
        Image {
            width: new_width,
            height: new_height,
            data: new_data,
            format: self.format,
        }
    }

    pub fn get_pixel(&self, x: u32, y: u32) -> (u8, u8, u8, u8) {
        if x < self.width && y < self.height {
            let idx = ((y * self.width + x) * 4) as usize;
            return (self.data[idx], self.data[idx+1], self.data[idx+2], self.data[idx+3]);
        }
        (0, 0, 0, 0)
    }

    pub fn set_pixel(&mut self, x: u32, y: u32, r: u8, g: u8, b: u8, a: u8) {
        if x < self.width && y < self.height {
            let idx = ((y * self.width + x) * 4) as usize;
            self.data[idx] = r;
            self.data[idx+1] = g;
            self.data[idx+2] = b;
            self.data[idx+3] = a;
        }
    }

    pub fn apply_filter(&mut self, filter: &Filter) {
        let mut new_data = self.data.clone();
        let pixels = self.data.chunks_exact(4);
        let new_pixels = new_data.chunks_exact_mut(4);
        for (src, dst) in pixels.zip(new_pixels) {
            let pixel = (src[0], src[1], src[2], src[3]);
            let result = filter.apply(pixel);
            dst[0] = result.0;
            dst[1] = result.1;
            dst[2] = result.2;
            dst[3] = result.3;
        }
        self.data = new_data;
    }
}

#[derive(Debug, Clone)]
pub enum Filter {
    Blur(f32),
    Brightness(f32),
    Contrast(f32),
    Grayscale(f32),
    Sepia(f32),
    HueRotate(f32),
    Invert(f32),
    Opacity(f32),
    Saturate(f32),
    DropShadow { dx: f32, dy: f32, blur: f32, color: (f32, f32, f32, f32) },
}

impl Filter {
    pub fn apply(&self, pixel: (u8, u8, u8, u8)) -> (u8, u8, u8, u8) {
        let (r, g, b, a) = pixel;
        let mut rf = r as f32 / 255.0;
        let mut gf = g as f32 / 255.0;
        let mut bf = b as f32 / 255.0;
        let af = a as f32 / 255.0;

        match self {
            Filter::Brightness(v) => {
                rf *= v;
                gf *= v;
                bf *= v;
            }
            Filter::Contrast(v) => {
                rf = ((rf - 0.5) * v) + 0.5;
                gf = ((gf - 0.5) * v) + 0.5;
                bf = ((bf - 0.5) * v) + 0.5;
            }
            Filter::Grayscale(v) => {
                let gray = rf * 0.299 + gf * 0.587 + bf * 0.114;
                rf = rf + (gray - rf) * v;
                gf = gf + (gray - gf) * v;
                bf = bf + (gray - bf) * v;
            }
            Filter::Sepia(v) => {
                let r2 = rf * 0.393 + gf * 0.769 + bf * 0.189;
                let g2 = rf * 0.349 + gf * 0.686 + bf * 0.168;
                let b2 = rf * 0.272 + gf * 0.534 + bf * 0.131;
                rf = rf + (r2 - rf) * v;
                gf = gf + (g2 - gf) * v;
                bf = bf + (b2 - bf) * v;
            }
            Filter::Invert(v) => {
                rf = (1.0 - rf) * v + rf * (1.0 - v);
                gf = (1.0 - gf) * v + gf * (1.0 - v);
                bf = (1.0 - bf) * v + bf * (1.0 - v);
            }
            Filter::Opacity(v) => {
                return ((rf * 255.0) as u8, (gf * 255.0) as u8, (bf * 255.0) as u8, (af * v * 255.0) as u8);
            }
            Filter::Saturate(v) => {
                let gray = rf * 0.299 + gf * 0.587 + bf * 0.114;
                rf = gray + (rf - gray) * v;
                gf = gray + (gf - gray) * v;
                bf = gray + (bf - gray) * v;
            }
            Filter::HueRotate(degrees) => {
                let angle = degrees.to_radians();
                let cos = angle.cos();
                let sin = angle.sin();
                let gray = rf * 0.299 + gf * 0.587 + bf * 0.114;
                let r_new = gray + (rf - gray) * cos + (gf - bf) * sin;
                let g_new = gray + (gf - gray) * cos + (bf - rf) * sin;
                let b_new = gray + (bf - gray) * cos + (rf - gf) * sin;
                rf = r_new;
                gf = g_new;
                bf = b_new;
            }
            Filter::Blur(radius) => {
                let blurred = Self::box_blur(pixel, *radius);
                return blurred;
            }
            Filter::DropShadow { dx, dy, blur, color } => {
                let (cr, cg, cb, ca) = *color;
                let shadow_r = (cr * 255.0) as u8;
                let shadow_g = (cg * 255.0) as u8;
                let shadow_b = (cb * 255.0) as u8;
                let shadow_a = (ca * af * 255.0) as u8;
                let alpha = (af * 255.0) as u8;
                let r_out = ((rf + cr * (1.0 - rf)) * 255.0) as u8;
                let g_out = ((gf + cg * (1.0 - gf)) * 255.0) as u8;
                let b_out = ((bf + cb * (1.0 - bf)) * 255.0) as u8;
                let a_out = ((af + ca * (1.0 - af)) * 255.0) as u8;
                return (r_out, g_out, b_out, a_out);
            }
        }

        let r = (rf.clamp(0.0, 1.0) * 255.0) as u8;
        let g = (gf.clamp(0.0, 1.0) * 255.0) as u8;
        let b = (bf.clamp(0.0, 1.0) * 255.0) as u8;
        (r, g, b, (af * 255.0) as u8)
    }

    fn box_blur(pixel: (u8, u8, u8, u8), radius: f32) -> (u8, u8, u8, u8) {
        if radius <= 0.0 {
            return pixel;
        }
        let r = pixel.0 as f32;
        let g = pixel.1 as f32;
        let b = pixel.2 as f32;
        let a = pixel.3 as f32;
        let spread = radius;
        let r_blur = r + (127.5 - r) * (1.0 - 1.0 / (1.0 + spread * 0.05));
        let g_blur = g + (127.5 - g) * (1.0 - 1.0 / (1.0 + spread * 0.05));
        let b_blur = b + (127.5 - b) * (1.0 - 1.0 / (1.0 + spread * 0.05));
        ((r_blur.clamp(0.0, 255.0)) as u8,
         (g_blur.clamp(0.0, 255.0)) as u8,
         (b_blur.clamp(0.0, 255.0)) as u8,
         pixel.3)
    }
}
