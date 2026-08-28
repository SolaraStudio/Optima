use rustybuzz::GlyphInfo;

#[derive(Debug, Clone, Copy)]
pub struct Glyph {
    pub glyph_id: u32,
    pub x_offset: f32,
    pub y_offset: f32,
    pub x_advance: f32,
    pub y_advance: f32,
    pub cluster: u32,
}

impl Glyph {
    pub fn new(glyph_id: u32, x_offset: f32, y_offset: f32, x_advance: f32, y_advance: f32, cluster: u32) -> Self {
        Self {
            glyph_id,
            x_offset,
            y_offset,
            x_advance,
            y_advance,
            cluster,
        }
    }

    pub fn from_info(info: GlyphInfo, font_size: f32) -> Self {
        let scale = font_size / 1000.0;
        Self {
            glyph_id: info.glyph_id,
            x_offset: info.x_offset as f32 * scale,
            y_offset: info.y_offset as f32 * scale,
            x_advance: info.x_advance as f32 * scale,
            y_advance: info.y_advance as f32 * scale,
            cluster: info.cluster,
        }
    }

    pub fn from_info_with_scale(info: GlyphInfo, scale: f32) -> Self {
        Self {
            glyph_id: info.glyph_id,
            x_offset: info.x_offset as f32 * scale,
            y_offset: info.y_offset as f32 * scale,
            x_advance: info.x_advance as f32 * scale,
            y_advance: info.y_advance as f32 * scale,
            cluster: info.cluster,
        }
    }

    pub fn width(&self) -> f32 {
        self.x_advance
    }

    pub fn height(&self) -> f32 {
        self.y_advance
    }

    pub fn advance(&self) -> (f32, f32) {
        (self.x_advance, self.y_advance)
    }
}

impl Default for Glyph {
    fn default() -> Self {
        Self {
            glyph_id: 0,
            x_offset: 0.0,
            y_offset: 0.0,
            x_advance: 0.0,
            y_advance: 0.0,
            cluster: 0,
        }
    }
}
