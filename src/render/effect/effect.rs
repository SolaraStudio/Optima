use vello::peniko::Color;

pub struct RenderEffect;

impl RenderEffect {
    pub fn blur_color(color: Color, blur: f32) -> Color {
        let factor = 1.0 / (1.0 + blur * 0.1);
        Color::new(
            color.r * factor,
            color.g * factor,
            color.b * factor,
            color.a,
        )
    }

    pub fn tint(color: Color, tint_color: Color, intensity: f32) -> Color {
        let r = color.r * (1.0 - intensity) + tint_color.r * intensity;
        let g = color.g * (1.0 - intensity) + tint_color.g * intensity;
        let b = color.b * (1.0 - intensity) + tint_color.b * intensity;
        Color::new(r, g, b, color.a)
    }
}
