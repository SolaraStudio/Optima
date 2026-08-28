#[derive(Debug, Clone)]
pub enum Filter {
    Blur(f32),
    Brightness(f32),
    Contrast(f32),
    Grayscale(f32),
    Sepia(f32),
    HueRotate(f32),
    Invert(f32),
    Opacity(f32),
    Saturate(f32),
    DropShadow { dx: f32, dy: f32, blur: f32, color: (f32, f32, f32, f32) },
}

impl Filter {
    pub fn apply(&self, pixel: (u8, u8, u8, u8)) -> (u8, u8, u8, u8) {
        let (r, g, b, a) = pixel;
        let mut rf = r as f32 / 255.0;
        let mut gf = g as f32 / 255.0;
        let mut bf = b as f32 / 255.0;
        let af = a as f32 / 255.0;

        match self {
            Filter::Brightness(v) => {
                rf *= v;
                gf *= v;
                bf *= v;
            }
            Filter::Contrast(v) => {
                rf = ((rf - 0.5) * v) + 0.5;
                gf = ((gf - 0.5) * v) + 0.5;
                bf = ((bf - 0.5) * v) + 0.5;
            }
            Filter::Grayscale(v) => {
                let gray = rf * 0.299 + gf * 0.587 + bf * 0.114;
                rf = rf + (gray - rf) * v;
                gf = gf + (gray - gf) * v;
                bf = bf + (gray - bf) * v;
            }
            Filter::Sepia(v) => {
                let r2 = rf * 0.393 + gf * 0.769 + bf * 0.189;
                let g2 = rf * 0.349 + gf * 0.686 + bf * 0.168;
                let b2 = rf * 0.272 + gf * 0.534 + bf * 0.131;
                rf = rf + (r2 - rf) * v;
                gf = gf + (g2 - gf) * v;
                bf = bf + (b2 - bf) * v;
            }
            Filter::Invert(v) => {
                rf = (1.0 - rf) * v + rf * (1.0 - v);
                gf = (1.0 - gf) * v + gf * (1.0 - v);
                bf = (1.0 - bf) * v + bf * (1.0 - v);
            }
            Filter::Opacity(v) => {
                return ((rf * 255.0) as u8, (gf * 255.0) as u8, (bf * 255.0) as u8, (af * v * 255.0) as u8);
            }
            Filter::Blur(_) | Filter::Saturate(_) | Filter::HueRotate(_) | Filter::DropShadow { .. } => {
                // These require more complex processing; placeholder
            }
        }

        let r = (rf.clamp(0.0, 1.0) * 255.0) as u8;
        let g = (gf.clamp(0.0, 1.0) * 255.0) as u8;
        let b = (bf.clamp(0.0, 1.0) * 255.0) as u8;
        (r, g, b, a)
    }
}
