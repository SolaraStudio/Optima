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
    pub fn apply_to_pixel(&self, pixel: (u8, u8, u8, u8)) -> (u8, u8, u8, u8) {
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
            Filter::Saturate(v) => {
                let gray = rf * 0.299 + gf * 0.587 + bf * 0.114;
                rf = gray + (rf - gray) * v;
                gf = gray + (gf - gray) * v;
                bf = gray + (bf - gray) * v;
            }
            Filter::HueRotate(degrees) => {
                let angle = degrees.to_radians();
                let cos = angle.cos();
                let sin = angle.sin();
                let gray = rf * 0.299 + gf * 0.587 + bf * 0.114;
                let r_new = gray + (rf - gray) * cos + (gf - bf) * sin;
                let g_new = gray + (gf - gray) * cos + (bf - rf) * sin;
                let b_new = gray + (bf - gray) * cos + (rf - gf) * sin;
                rf = r_new;
                gf = g_new;
                bf = b_new;
            }
            Filter::Blur(radius) => {
                let spread = radius.clamp(0.0, 20.0);
                let factor = 1.0 / (1.0 + spread * 0.08);
                rf = rf * factor + 0.5 * (1.0 - factor);
                gf = gf * factor + 0.5 * (1.0 - factor);
                bf = bf * factor + 0.5 * (1.0 - factor);
            }
            Filter::DropShadow { dx, dy, blur, color } => {
                let (cr, cg, cb, ca) = *color;
                let shadow_r = cr * 255.0;
                let shadow_g = cg * 255.0;
                let shadow_b = cb * 255.0;
                let shadow_a = ca * af * 255.0;
                let spread = blur.clamp(0.0, 20.0);
                let factor = 1.0 / (1.0 + spread * 0.08);
                let blur_r = rf * factor + 0.5 * (1.0 - factor);
                let blur_g = gf * factor + 0.5 * (1.0 - factor);
                let blur_b = bf * factor + 0.5 * (1.0 - factor);
                let r_out = ((blur_r * (1.0 - ca) + cr * ca) * 255.0) as u8;
                let g_out = ((blur_g * (1.0 - ca) + cg * ca) * 255.0) as u8;
                let b_out = ((blur_b * (1.0 - ca) + cb * ca) * 255.0) as u8;
                let a_out = ((af * 255.0) as f32 * (1.0 - ca) + ca * 255.0) as u8;
                return (r_out, g_out, b_out, a_out);
            }
        }

        let r = (rf.clamp(0.0, 1.0) * 255.0) as u8;
        let g = (gf.clamp(0.0, 1.0) * 255.0) as u8;
        let b = (bf.clamp(0.0, 1.0) * 255.0) as u8;
        (r, g, b, (af * 255.0) as u8)
    }

    pub fn apply_to_image(&self, image: &mut crate::render::image::Image) {
        let mut new_data = image.data.clone();
        let pixels = image.data.chunks_exact(4);
        let new_pixels = new_data.chunks_exact_mut(4);
        for (src, dst) in pixels.zip(new_pixels) {
            let pixel = (src[0], src[1], src[2], src[3]);
            let result = self.apply_to_pixel(pixel);
            dst[0] = result.0;
            dst[1] = result.1;
            dst[2] = result.2;
            dst[3] = result.3;
        }
        image.data = new_data;
    }
}
