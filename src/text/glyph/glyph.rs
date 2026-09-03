#[derive(Debug, Clone, Default)]
pub struct Glyph {
    pub id: u32,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub advance_x: f32,
    pub advance_y: f32,
    pub offset_x: f32,
    pub offset_y: f32,
}

impl Glyph {
    pub fn new(id: u32) -> Self {
        Glyph {
            id,
            ..Default::default()
        }
    }

    pub fn with_position(mut self, x: f32, y: f32) -> Self {
        self.x = x;
        self.y = y;
        self
    }
    pub fn with_size(mut self, w: f32, h: f32) -> Self {
        self.width = w;
        self.height = h;
        self
    }
    pub fn with_advance(mut self, ax: f32, ay: f32) -> Self {
        self.advance_x = ax;
        self.advance_y = ay;
        self
    }
    pub fn with_offset(mut self, ox: f32, oy: f32) -> Self {
        self.offset_x = ox;
        self.offset_y = oy;
        self
    }
}

#[derive(Debug, Clone, Default)]
pub struct GlyphRun {
    pub glyphs: Vec<Glyph>,
    pub start_offset: f32,
    pub end_offset: f32,
    pub font_size: f32,
}

impl GlyphRun {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_glyph(&mut self, glyph: Glyph) {
        self.end_offset += glyph.advance_x;
        self.glyphs.push(glyph);
    }

    pub fn total_advance(&self) -> f32 {
        self.end_offset - self.start_offset
    }
}
