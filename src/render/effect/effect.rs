#[derive(Debug, Clone)]
pub enum Effect {
    Blur(f32),
    Brightness(f32),
    Contrast(f32),
    Grayscale(f32),
    Invert(f32),
    Opacity(f32),
    Saturate(f32),
    Sepia(f32),
    DropShadow {
        x: f32,
        y: f32,
        blur: f32,
        color: (f32, f32, f32, f32),
    },
}

impl Effect {
    pub fn blur(radius: f32) -> Self {
        Effect::Blur(radius)
    }
    pub fn brightness(v: f32) -> Self {
        Effect::Brightness(v)
    }
    pub fn opacity(v: f32) -> Self {
        Effect::Opacity(v)
    }
    pub fn drop_shadow(x: f32, y: f32, blur: f32, color: (f32, f32, f32, f32)) -> Self {
        Effect::DropShadow { x, y, blur, color }
    }
}
