#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[derive(Default)]
pub enum ScreenshotFormat {
    #[default]
    Rgba8,
    Rgba16,
    Bgra8,
    Rgb8,
}


impl ScreenshotFormat {
    pub fn bytes_per_pixel(&self) -> usize {
        match self {
            ScreenshotFormat::Rgba8 => 4,
            ScreenshotFormat::Rgba16 => 8,
            ScreenshotFormat::Bgra8 => 4,
            ScreenshotFormat::Rgb8 => 3,
        }
    }

    pub fn channel_count(&self) -> usize {
        match self {
            ScreenshotFormat::Rgba8 => 4,
            ScreenshotFormat::Rgba16 => 4,
            ScreenshotFormat::Bgra8 => 4,
            ScreenshotFormat::Rgb8 => 3,
        }
    }
}

#[derive(Debug, Clone)]
pub struct PixelBuffer {
    pub data: Vec<u8>,
    pub width: u32,
    pub height: u32,
    pub format: ScreenshotFormat,
    pub stride: usize,
}

impl Default for PixelBuffer {
    fn default() -> Self {
        PixelBuffer {
            data: Vec::new(),
            width: 0,
            height: 0,
            format: ScreenshotFormat::Rgba8,
            stride: 0,
        }
    }
}

impl PixelBuffer {
    pub fn new(width: u32, height: u32, format: ScreenshotFormat) -> Self {
        let bpp = format.bytes_per_pixel();
        let stride = width as usize * bpp;
        let data = vec![0u8; stride * height as usize];
        PixelBuffer {
            data,
            width,
            height,
            format,
            stride,
        }
    }

    pub fn from_raw(
        data: Vec<u8>,
        width: u32,
        height: u32,
        format: ScreenshotFormat,
    ) -> Result<Self, String> {
        let bpp = format.bytes_per_pixel();
        let expected = width as usize * height as usize * bpp;
        if data.len() != expected {
            return Err(format!(
                "Data length {} does not match expected {} for {}x{} {:?}",
                data.len(),
                expected,
                width,
                height,
                format,
            ));
        }
        let stride = width as usize * bpp;
        Ok(PixelBuffer {
            data,
            width,
            height,
            format,
            stride,
        })
    }

    pub fn pixel_count(&self) -> usize {
        self.width as usize * self.height as usize
    }

    pub fn byte_count(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn get_pixel(&self, x: u32, y: u32) -> Option<&[u8]> {
        if x >= self.width || y >= self.height {
            return None;
        }
        let bpp = self.format.bytes_per_pixel();
        let offset = (y as usize * self.stride) + (x as usize * bpp);
        if offset + bpp <= self.data.len() {
            Some(&self.data[offset..offset + bpp])
        } else {
            None
        }
    }

    pub fn set_pixel(&mut self, x: u32, y: u32, color: &[u8]) -> Result<(), String> {
        if x >= self.width || y >= self.height {
            return Err(format!(
                "Pixel ({}, {}) out of bounds for {}x{}",
                x, y, self.width, self.height
            ));
        }
        let bpp = self.format.bytes_per_pixel();
        if color.len() != bpp {
            return Err(format!(
                "Color data has {} bytes, expected {}",
                color.len(),
                bpp
            ));
        }
        let offset = (y as usize * self.stride) + (x as usize * bpp);
        self.data[offset..offset + bpp].copy_from_slice(color);
        Ok(())
    }

    pub fn fill(&mut self, color: &[u8]) -> Result<(), String> {
        let bpp = self.format.bytes_per_pixel();
        if color.len() != bpp {
            return Err(format!(
                "Color data has {} bytes, expected {}",
                color.len(),
                bpp
            ));
        }
        for y in 0..self.height {
            for x in 0..self.width {
                let offset = (y as usize * self.stride) + (x as usize * bpp);
                self.data[offset..offset + bpp].copy_from_slice(color);
            }
        }
        Ok(())
    }

    pub fn sub_region(&self, x: u32, y: u32, w: u32, h: u32) -> Result<PixelBuffer, String> {
        if x + w > self.width || y + h > self.height {
            return Err("Region exceeds buffer bounds".to_string());
        }
        let bpp = self.format.bytes_per_pixel();
        let mut sub_data = Vec::with_capacity(w as usize * h as usize * bpp);
        for row in y..y + h {
            let start = row as usize * self.stride + x as usize * bpp;
            let end = start + w as usize * bpp;
            sub_data.extend_from_slice(&self.data[start..end]);
        }
        Ok(PixelBuffer {
            data: sub_data,
            width: w,
            height: h,
            format: self.format,
            stride: w as usize * bpp,
        })
    }

    pub fn flip_vertical(&mut self) {
        let bpp = self.format.bytes_per_pixel();
        let row_size = self.width as usize * bpp;
        for y in 0..self.height / 2 {
            let top_start = y as usize * row_size;
            let bot_start = (self.height - 1 - y) as usize * row_size;
            for i in 0..row_size {
                self.data.swap(top_start + i, bot_start + i);
            }
        }
    }

    pub fn premultiply_alpha(&mut self) {
        let bpp = self.format.bytes_per_pixel();
        if bpp < 4 {
            return;
        }
        for chunk in self.data.chunks_exact_mut(bpp) {
            let alpha = chunk[3] as f32 / 255.0;
            chunk[0] = (chunk[0] as f32 * alpha) as u8;
            chunk[1] = (chunk[1] as f32 * alpha) as u8;
            chunk[2] = (chunk[2] as f32 * alpha) as u8;
        }
    }
}

#[derive(Debug, Clone)]
#[derive(Default)]
pub struct ScreenshotCapture {
    pub buffer: PixelBuffer,
    pub timestamp_ms: u64,
    pub frame_number: u64,
    pub label: String,
}


impl ScreenshotCapture {
    pub fn new(width: u32, height: u32, format: ScreenshotFormat) -> Self {
        ScreenshotCapture {
            buffer: PixelBuffer::new(width, height, format),
            timestamp_ms: 0,
            frame_number: 0,
            label: String::new(),
        }
    }

    pub fn with_label(mut self, label: &str) -> Self {
        self.label = label.to_string();
        self
    }

    pub fn with_timestamp(mut self, ms: u64) -> Self {
        self.timestamp_ms = ms;
        self
    }

    pub fn with_frame(mut self, frame: u64) -> Self {
        self.frame_number = frame;
        self
    }

    pub fn pixel_count(&self) -> usize {
        self.buffer.pixel_count()
    }

    pub fn dimensions(&self) -> (u32, u32) {
        (self.buffer.width, self.buffer.height)
    }

    pub fn avg_luminance(&self) -> f32 {
        if self.buffer.data.is_empty() {
            return 0.0;
        }
        let bpp = self.buffer.format.bytes_per_pixel();
        if bpp < 3 {
            return 0.0;
        }
        let mut total: f64 = 0.0;
        let mut count: usize = 0;
        for chunk in self.buffer.data.chunks_exact(bpp) {
            let r = chunk[0] as f64 / 255.0;
            let g = chunk[1] as f64 / 255.0;
            let b = chunk[2] as f64 / 255.0;
            total += 0.299 * r + 0.587 * g + 0.114 * b;
            count += 1;
        }
        if count > 0 {
            (total / count as f64) as f32
        } else {
            0.0
        }
    }

    pub fn compare(&self, other: &ScreenshotCapture) -> f32 {
        if self.buffer.width != other.buffer.width || self.buffer.height != other.buffer.height {
            return 0.0;
        }
        if self.buffer.data.len() != other.buffer.data.len() {
            return 0.0;
        }
        if self.buffer.data.is_empty() {
            return 1.0;
        }
        let mut diff_sum: f64 = 0.0;
        let len = self.buffer.data.len();
        for i in 0..len {
            let a = self.buffer.data[i] as f64;
            let b = other.buffer.data[i] as f64;
            diff_sum += (a - b).abs();
        }
        let max_diff = len as f64 * 255.0;
        (1.0 - diff_sum / max_diff) as f32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_screenshot_format() {
        assert_eq!(ScreenshotFormat::Rgba8.bytes_per_pixel(), 4);
        assert_eq!(ScreenshotFormat::Rgba16.bytes_per_pixel(), 8);
        assert_eq!(ScreenshotFormat::Rgb8.bytes_per_pixel(), 3);
        assert_eq!(ScreenshotFormat::Bgra8.bytes_per_pixel(), 4);
        assert_eq!(ScreenshotFormat::Rgba8.channel_count(), 4);
        assert_eq!(ScreenshotFormat::Rgb8.channel_count(), 3);
    }

    #[test]
    fn test_pixel_buffer_new() {
        let buf = PixelBuffer::new(100, 50, ScreenshotFormat::Rgba8);
        assert_eq!(buf.width, 100);
        assert_eq!(buf.height, 50);
        assert_eq!(buf.stride, 400);
        assert_eq!(buf.pixel_count(), 5000);
        assert_eq!(buf.byte_count(), 20000);
    }

    #[test]
    fn test_pixel_buffer_from_raw() {
        let data = vec![0u8; 10 * 10 * 4];
        let buf = PixelBuffer::from_raw(data, 10, 10, ScreenshotFormat::Rgba8).unwrap();
        assert_eq!(buf.pixel_count(), 100);

        let bad = vec![0u8; 5];
        let result = PixelBuffer::from_raw(bad, 10, 10, ScreenshotFormat::Rgba8);
        assert!(result.is_err());
    }

    #[test]
    fn test_pixel_buffer_get_set() {
        let mut buf = PixelBuffer::new(4, 4, ScreenshotFormat::Rgba8);
        let pixel = [255, 128, 0, 200];
        buf.set_pixel(2, 3, &pixel).unwrap();

        let got = buf.get_pixel(2, 3).unwrap();
        assert_eq!(got, &pixel);

        assert!(buf.get_pixel(10, 10).is_none());
        assert!(buf.set_pixel(10, 10, &pixel).is_err());
    }

    #[test]
    fn test_pixel_buffer_fill() {
        let mut buf = PixelBuffer::new(10, 10, ScreenshotFormat::Rgba8);
        let red = [255, 0, 0, 255];
        buf.fill(&red).unwrap();
        assert_eq!(buf.get_pixel(5, 5).unwrap(), &red);
        assert_eq!(buf.get_pixel(0, 0).unwrap(), &red);

        let bad = [0u8; 3];
        assert!(buf.fill(&bad).is_err());
    }

    #[test]
    fn test_pixel_buffer_sub_region() {
        let mut buf = PixelBuffer::new(10, 10, ScreenshotFormat::Rgba8);
        let blue = [0, 0, 255, 255];
        buf.set_pixel(5, 5, &blue).unwrap();

        let sub = buf.sub_region(4, 4, 3, 3).unwrap();
        assert_eq!(sub.width, 3);
        assert_eq!(sub.height, 3);
        assert_eq!(sub.get_pixel(1, 1).unwrap(), &blue);

        assert!(buf.sub_region(8, 8, 5, 5).is_err());
    }

    #[test]
    fn test_pixel_buffer_flip_vertical() {
        let mut buf = PixelBuffer::new(2, 4, ScreenshotFormat::Rgba8);
        let c1 = [10, 0, 0, 255];
        let c2 = [20, 0, 0, 255];
        buf.set_pixel(0, 0, &c1).unwrap();
        buf.set_pixel(0, 3, &c2).unwrap();

        buf.flip_vertical();
        assert_eq!(buf.get_pixel(0, 3).unwrap(), &c1);
        assert_eq!(buf.get_pixel(0, 0).unwrap(), &c2);
    }

    #[test]
    fn test_pixel_buffer_premultiply_alpha() {
        let mut buf = PixelBuffer::new(1, 1, ScreenshotFormat::Rgba8);
        buf.set_pixel(0, 0, &[200, 100, 50, 128]).unwrap();
        buf.premultiply_alpha();

        let pixel = buf.get_pixel(0, 0).unwrap();
        assert_eq!(pixel[0], (200.0 * 128.0 / 255.0) as u8);
        assert_eq!(pixel[1], (100.0 * 128.0 / 255.0) as u8);
        assert_eq!(pixel[2], (50.0 * 128.0 / 255.0) as u8);
        assert_eq!(pixel[3], 128);
    }

    #[test]
    fn test_screenshot_capture() {
        let cap = ScreenshotCapture::new(1920, 1080, ScreenshotFormat::Rgba8)
            .with_label("test")
            .with_timestamp(1234)
            .with_frame(42);
        assert_eq!(cap.dimensions(), (1920, 1080));
        assert_eq!(cap.label, "test");
        assert_eq!(cap.timestamp_ms, 1234);
        assert_eq!(cap.frame_number, 42);
        assert_eq!(cap.pixel_count(), 1920 * 1080);
    }

    #[test]
    fn test_screenshot_capture_avg_luminance() {
        let mut cap = ScreenshotCapture::new(2, 2, ScreenshotFormat::Rgba8);
        cap.buffer.fill(&[255, 255, 255, 255]).unwrap();
        let lum = cap.avg_luminance();
        assert!(lum > 0.99);

        let empty = ScreenshotCapture::default();
        assert_eq!(empty.avg_luminance(), 0.0);
    }

    #[test]
    fn test_screenshot_compare() {
        let a = ScreenshotCapture::new(2, 2, ScreenshotFormat::Rgba8);
        let b = ScreenshotCapture::new(2, 2, ScreenshotFormat::Rgba8);
        assert!((a.compare(&b) - 1.0).abs() < 0.001);

        let c = ScreenshotCapture::new(4, 4, ScreenshotFormat::Rgba8);
        assert_eq!(a.compare(&c), 0.0);

        let d = ScreenshotCapture::default();
        assert_eq!(a.compare(&d), 0.0);
    }
}
