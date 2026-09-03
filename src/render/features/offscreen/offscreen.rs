#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[derive(Default)]
pub enum OffscreenFormat {
    #[default]
    Rgba8,
    Rgba16Float,
    Bgra8,
    Depth24Stencil8,
}


impl OffscreenFormat {
    pub fn bytes_per_pixel(&self) -> usize {
        match self {
            OffscreenFormat::Rgba8 => 4,
            OffscreenFormat::Rgba16Float => 8,
            OffscreenFormat::Bgra8 => 4,
            OffscreenFormat::Depth24Stencil8 => 4,
        }
    }

    pub fn is_depth(&self) -> bool {
        matches!(self, OffscreenFormat::Depth24Stencil8)
    }
}

#[derive(Debug, Clone)]
pub struct OffscreenTexture {
    pub width: u32,
    pub height: u32,
    pub format: OffscreenFormat,
    pub data: Vec<u8>,
    pub dirty: bool,
    pub sample_count: u32,
}

impl Default for OffscreenTexture {
    fn default() -> Self {
        OffscreenTexture {
            width: 0,
            height: 0,
            format: OffscreenFormat::Rgba8,
            data: Vec::new(),
            dirty: false,
            sample_count: 1,
        }
    }
}

impl OffscreenTexture {
    pub fn new(width: u32, height: u32, format: OffscreenFormat) -> Self {
        let bpp = format.bytes_per_pixel();
        let size = width as usize * height as usize * bpp;
        OffscreenTexture {
            width,
            height,
            format,
            data: vec![0u8; size],
            dirty: false,
            sample_count: 1,
        }
    }

    pub fn with_sample_count(mut self, count: u32) -> Self {
        self.sample_count = count.clamp(1, 8);
        self
    }

    pub fn byte_count(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        if self.width == width && self.height == height {
            return;
        }
        let bpp = self.format.bytes_per_pixel();
        let new_size = width as usize * height as usize * bpp;
        self.data.resize(new_size, 0);
        self.width = width;
        self.height = height;
        self.dirty = true;
    }

    pub fn clear(&mut self) {
        self.data.fill(0);
        self.dirty = true;
    }

    pub fn fill(&mut self, value: u8) {
        self.data.fill(value);
        self.dirty = true;
    }

    pub fn read_pixel(&self, x: u32, y: u32) -> Option<&[u8]> {
        if x >= self.width || y >= self.height {
            return None;
        }
        let bpp = self.format.bytes_per_pixel();
        let offset = (y as usize * self.width as usize * bpp) + (x as usize * bpp);
        if offset + bpp <= self.data.len() {
            Some(&self.data[offset..offset + bpp])
        } else {
            None
        }
    }

    pub fn write_pixel(&mut self, x: u32, y: u32, data: &[u8]) -> Result<(), String> {
        if x >= self.width || y >= self.height {
            return Err("Out of bounds".to_string());
        }
        let bpp = self.format.bytes_per_pixel();
        if data.len() < bpp {
            return Err("Insufficient data".to_string());
        }
        let offset = (y as usize * self.width as usize * bpp) + (x as usize * bpp);
        self.data[offset..offset + bpp].copy_from_slice(&data[..bpp]);
        self.dirty = true;
        Ok(())
    }

    pub fn copy_from(&mut self, other: &OffscreenTexture) -> Result<(), String> {
        if self.width != other.width || self.height != other.height || self.format != other.format {
            return Err("Texture dimensions or format mismatch".to_string());
        }
        self.data.copy_from_slice(&other.data);
        self.dirty = true;
        Ok(())
    }

    pub fn mark_clean(&mut self) {
        self.dirty = false;
    }

    pub fn region(&self, x: u32, y: u32, w: u32, h: u32) -> Result<OffscreenTexture, String> {
        if x + w > self.width || y + h > self.height {
            return Err("Region out of bounds".to_string());
        }
        let bpp = self.format.bytes_per_pixel();
        let row_bytes = w as usize * bpp;
        let mut tex = OffscreenTexture::new(w, h, self.format);
        for row in 0..h {
            let src_offset = ((y + row) as usize * self.width as usize * bpp) + (x as usize * bpp);
            let dst_offset = row as usize * row_bytes;
            tex.data[dst_offset..dst_offset + row_bytes]
                .copy_from_slice(&self.data[src_offset..src_offset + row_bytes]);
        }
        tex.dirty = true;
        Ok(tex)
    }

    pub fn aspect_ratio(&self) -> f32 {
        if self.height == 0 {
            return 0.0;
        }
        self.width as f32 / self.height as f32
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[derive(Default)]
pub enum OffscreenBlendMode {
    #[default]
    Replace,
    AlphaBlend,
    Additive,
    Multiply,
}


#[derive(Debug, Clone)]
#[derive(Default)]
pub struct OffscreenRenderTarget {
    pub color_texture: OffscreenTexture,
    pub depth_texture: Option<OffscreenTexture>,
    pub blend_mode: OffscreenBlendMode,
    pub label: String,
    pub frame_count: u64,
    pub total_pixels_rendered: u64,
}


impl OffscreenRenderTarget {
    pub fn new(width: u32, height: u32) -> Self {
        OffscreenRenderTarget {
            color_texture: OffscreenTexture::new(width, height, OffscreenFormat::Rgba8),
            ..Default::default()
        }
    }

    pub fn with_format(mut self, format: OffscreenFormat) -> Self {
        self.color_texture =
            OffscreenTexture::new(self.color_texture.width, self.color_texture.height, format);
        self
    }

    pub fn with_depth(mut self) -> Self {
        let w = self.color_texture.width;
        let h = self.color_texture.height;
        self.depth_texture = Some(OffscreenTexture::new(
            w,
            h,
            OffscreenFormat::Depth24Stencil8,
        ));
        self
    }

    pub fn with_label(mut self, label: &str) -> Self {
        self.label = label.to_string();
        self
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.color_texture.resize(width, height);
        if let Some(ref mut depth) = self.depth_texture {
            depth.resize(width, height);
        }
    }

    pub fn begin_frame(&mut self) {
        self.color_texture.clear();
        if let Some(ref mut depth) = self.depth_texture {
            depth.clear();
        }
        self.frame_count += 1;
    }

    pub fn end_frame(&mut self) {
        let pixels = self.color_texture.width as u64 * self.color_texture.height as u64;
        self.total_pixels_rendered += pixels;
        self.color_texture.dirty = true;
    }

    pub fn dimensions(&self) -> (u32, u32) {
        (self.color_texture.width, self.color_texture.height)
    }

    pub fn read_pixels(&self) -> &[u8] {
        &self.color_texture.data
    }

    pub fn average_pixel_value(&self) -> f32 {
        if self.color_texture.data.is_empty() {
            return 0.0;
        }
        let sum: u64 = self.color_texture.data.iter().map(|&b| b as u64).sum();
        sum as f32 / self.color_texture.data.len() as f32
    }

    pub fn has_content(&self) -> bool {
        self.color_texture.data.iter().any(|&b| b != 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_offscreen_format() {
        assert_eq!(OffscreenFormat::Rgba8.bytes_per_pixel(), 4);
        assert_eq!(OffscreenFormat::Rgba16Float.bytes_per_pixel(), 8);
        assert!(!OffscreenFormat::Rgba8.is_depth());
        assert!(OffscreenFormat::Depth24Stencil8.is_depth());
    }

    #[test]
    fn test_offscreen_texture() {
        let tex = OffscreenTexture::new(100, 100, OffscreenFormat::Rgba8);
        assert_eq!(tex.width, 100);
        assert_eq!(tex.height, 100);
        assert_eq!(tex.byte_count(), 40000);
        assert!(!tex.is_empty());
    }

    #[test]
    fn test_offscreen_texture_read_write() {
        let mut tex = OffscreenTexture::new(10, 10, OffscreenFormat::Rgba8);
        let color = [255, 128, 64, 200];
        tex.write_pixel(5, 5, &color).unwrap();
        assert_eq!(tex.read_pixel(5, 5).unwrap(), &color);

        assert!(tex.read_pixel(20, 20).is_none());
        assert!(tex.write_pixel(20, 20, &color).is_err());
    }

    #[test]
    fn test_offscreen_texture_resize() {
        let mut tex = OffscreenTexture::new(10, 10, OffscreenFormat::Rgba8);
        tex.resize(20, 20);
        assert_eq!(tex.width, 20);
        assert_eq!(tex.height, 20);
        assert!(tex.dirty);

        tex.resize(20, 20);
        assert_eq!(tex.width, 20);
    }

    #[test]
    fn test_offscreen_texture_copy() {
        let mut a = OffscreenTexture::new(5, 5, OffscreenFormat::Rgba8);
        let mut b = OffscreenTexture::new(5, 5, OffscreenFormat::Rgba8);
        b.fill(128);

        a.copy_from(&b).unwrap();
        assert_eq!(a.data[0], 128);

        let c = OffscreenTexture::new(10, 10, OffscreenFormat::Rgba8);
        assert!(a.copy_from(&c).is_err());
    }

    #[test]
    fn test_offscreen_texture_region() {
        let mut tex = OffscreenTexture::new(10, 10, OffscreenFormat::Rgba8);
        tex.fill(42);
        let sub = tex.region(2, 2, 3, 3).unwrap();
        assert_eq!(sub.width, 3);
        assert_eq!(sub.height, 3);
        assert!(sub.data.iter().all(|&b| b == 42));

        assert!(tex.region(8, 8, 5, 5).is_err());
    }

    #[test]
    fn test_offscreen_texture_sample_count() {
        let tex = OffscreenTexture::new(10, 10, OffscreenFormat::Rgba8).with_sample_count(4);
        assert_eq!(tex.sample_count, 4);

        let tex2 = OffscreenTexture::new(10, 10, OffscreenFormat::Rgba8).with_sample_count(16);
        assert_eq!(tex2.sample_count, 8);
    }

    #[test]
    fn test_offscreen_render_target() {
        let mut rt = OffscreenRenderTarget::new(1920, 1080)
            .with_label("main")
            .with_depth();

        assert_eq!(rt.dimensions(), (1920, 1080));
        assert!(rt.depth_texture.is_some());
        assert_eq!(rt.label, "main");

        rt.begin_frame();
        assert_eq!(rt.frame_count, 1);
        assert!(!rt.has_content());

        rt.end_frame();
        assert_eq!(rt.total_pixels_rendered, 1920 * 1080);
    }

    #[test]
    fn test_render_target_resize() {
        let mut rt = OffscreenRenderTarget::new(100, 100).with_depth();
        rt.resize(200, 200);
        assert_eq!(rt.dimensions(), (200, 200));
        assert_eq!(rt.depth_texture.as_ref().unwrap().width, 200);
    }

    #[test]
    fn test_render_target_with_format() {
        let rt = OffscreenRenderTarget::new(64, 64).with_format(OffscreenFormat::Rgba16Float);
        assert_eq!(rt.color_texture.format, OffscreenFormat::Rgba16Float);
    }

    #[test]
    fn test_average_pixel_value() {
        let mut rt = OffscreenRenderTarget::new(2, 2);
        rt.begin_frame();
        assert_eq!(rt.average_pixel_value(), 0.0);

        for i in 0..rt.color_texture.data.len() {
            rt.color_texture.data[i] = 100;
        }
        assert!((rt.average_pixel_value() - 100.0).abs() < 0.01);
    }

    #[test]
    fn test_has_content() {
        let mut rt = OffscreenRenderTarget::new(2, 2);
        assert!(!rt.has_content());
        rt.color_texture.data[0] = 1;
        assert!(rt.has_content());
    }
}
