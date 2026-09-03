use crate::css::colors::Color;

pub struct BlendOps;

impl BlendOps {
    pub fn blend_normal(base: Color, overlay: Color) -> Color { overlay }
    pub fn blend_multiply(base: Color, overlay: Color) -> Color {
        Color::new(base.r*overlay.r, base.g*overlay.g, base.b*overlay.b, base.a*overlay.a)
    }
    pub fn blend_screen(base: Color, overlay: Color) -> Color {
        Color::new(1.0-(1.0-base.r)*(1.0-overlay.r), 1.0-(1.0-base.g)*(1.0-overlay.g), 1.0-(1.0-base.b)*(1.0-overlay.b), base.a*overlay.a)
    }
    pub fn blend_overlay(base: Color, overlay: Color) -> Color {
        Color::new(
            if base.r < 0.5 { 2.0*base.r*overlay.r } else { 1.0 - 2.0*(1.0-base.r)*(1.0-overlay.r) },
            if base.g < 0.5 { 2.0*base.g*overlay.g } else { 1.0 - 2.0*(1.0-base.g)*(1.0-overlay.g) },
            if base.b < 0.5 { 2.0*base.b*overlay.b } else { 1.0 - 2.0*(1.0-base.b)*(1.0-overlay.b) },
            base.a * overlay.a,
        )
    }
    pub fn blend_darken(base: Color, overlay: Color) -> Color {
        Color::new(base.r.min(overlay.r), base.g.min(overlay.g), base.b.min(overlay.b), base.a)
    }
    pub fn blend_lighten(base: Color, overlay: Color) -> Color {
        Color::new(base.r.max(overlay.r), base.g.max(overlay.g), base.b.max(overlay.b), base.a)
    }
}
