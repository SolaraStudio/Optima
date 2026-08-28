use vello::peniko::Color;

pub struct Paint;

impl Paint {
    pub fn color(r: f64, g: f64, b: f64, a: f64) -> Color {
        Color::new(r as f32, g as f32, b as f32, a as f32)
    }

    pub fn hex(hex: u32) -> Color {
        let r = ((hex >> 16) & 0xFF) as f32 / 255.0;
        let g = ((hex >> 8) & 0xFF) as f32 / 255.0;
        let b = (hex & 0xFF) as f32 / 255.0;
        Color::new(r, g, b, 1.0)
    }

    pub fn white() -> Color {
        Color::WHITE
    }

    pub fn black() -> Color {
        Color::BLACK
    }

    pub fn purple() -> Color {
        Color::new(0.5, 0.2, 0.8, 1.0)
    }
}
