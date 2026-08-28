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
    Hue,
    Saturation,
    Color,
    Luminosity,
}

impl BlendMode {
    pub fn blend(&self, src: (f32, f32, f32, f32), dst: (f32, f32, f32, f32)) -> (f32, f32, f32, f32) {
        let (sr, sg, sb, sa) = src;
        let (dr, dg, db, da) = dst;
        let out_a = sa + da * (1.0 - sa);
        if out_a == 0.0 {
            return (0.0, 0.0, 0.0, 0.0);
        }
        let blend = match self {
            BlendMode::Normal => (sr, sg, sb),
            BlendMode::Multiply => (sr * dr, sg * dg, sb * db),
            BlendMode::Screen => (1.0 - (1.0 - sr) * (1.0 - dr), 1.0 - (1.0 - sg) * (1.0 - dg), 1.0 - (1.0 - sb) * (1.0 - db)),
            BlendMode::Overlay => {
                let blend_channel = |s: f32, d: f32| if d < 0.5 { 2.0 * s * d } else { 1.0 - 2.0 * (1.0 - s) * (1.0 - d) };
                (blend_channel(sr, dr), blend_channel(sg, dg), blend_channel(sb, db))
            }
            BlendMode::Darken => (sr.min(dr), sg.min(dg), sb.min(db)),
            BlendMode::Lighten => (sr.max(dr), sg.max(dg), sb.max(db)),
            _ => (sr, sg, sb),
        };
        let out_r = (blend.0 * sa + dr * da * (1.0 - sa)) / out_a;
        let out_g = (blend.1 * sa + dg * da * (1.0 - sa)) / out_a;
        let out_b = (blend.2 * sa + db * da * (1.0 - sa)) / out_a;
        (out_r, out_g, out_b, out_a)
    }
}
