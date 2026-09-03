use crate::css::colors::Color;
use crate::css::units::Length;

#[derive(Debug, Clone)]
pub enum PaintStyle {
    Fill(Color),
    Stroke { color: Color, width: f32 },
    LinearGradient { colors: Vec<Color>, angle: f32 },
    RadialGradient { colors: Vec<Color> },
    Image { data: Vec<u8>, width: u32, height: u32 },
}

impl Default for PaintStyle {
    fn default() -> Self { PaintStyle::Fill(Color::default()) }
}

#[derive(Debug, Clone)]
pub struct PaintCommand {
    pub style: PaintStyle,
    pub opacity: f32,
    pub blend_mode: BlendMode,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BlendMode {
    Normal,
    Multiply,
    Screen,
    Overlay,
    Darken,
    Lighten,
    ColorDodge,
    ColorBurn,
    HardLight,
    SoftLight,
    Difference,
    Exclusion,
}

impl Default for BlendMode {
    fn default() -> Self { BlendMode::Normal }
}

impl PaintCommand {
    pub fn fill(color: Color) -> Self {
        PaintCommand { style: PaintStyle::Fill(color), opacity: 1.0, blend_mode: BlendMode::Normal }
    }

    pub fn stroke(color: Color, width: f32) -> Self {
        PaintCommand { style: PaintStyle::Stroke { color, width }, opacity: 1.0, blend_mode: BlendMode::Normal }
    }

    pub fn with_opacity(mut self, opacity: f32) -> Self { self.opacity = opacity; self }
}
