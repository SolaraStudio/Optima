#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[derive(Default)]
pub enum CompositingBlendMode {
    #[default]
    Normal,
    Multiply,
    Screen,
    Overlay,
    Additive,
}


#[derive(Debug, Clone, Copy)]
pub struct LayerTransform {
    pub translate_x: f32,
    pub translate_y: f32,
    pub scale_x: f32,
    pub scale_y: f32,
    pub rotation_deg: f32,
    pub skew_x: f32,
    pub skew_y: f32,
}

impl Default for LayerTransform {
    fn default() -> Self {
        LayerTransform {
            translate_x: 0.0,
            translate_y: 0.0,
            scale_x: 1.0,
            scale_y: 1.0,
            rotation_deg: 0.0,
            skew_x: 0.0,
            skew_y: 0.0,
        }
    }
}

impl LayerTransform {
    pub fn identity() -> Self {
        Self::default()
    }

    pub fn translate(x: f32, y: f32) -> Self {
        LayerTransform {
            translate_x: x,
            translate_y: y,
            ..Default::default()
        }
    }

    pub fn scale(sx: f32, sy: f32) -> Self {
        LayerTransform {
            scale_x: sx,
            scale_y: sy,
            ..Default::default()
        }
    }

    pub fn rotation(deg: f32) -> Self {
        LayerTransform {
            rotation_deg: deg,
            ..Default::default()
        }
    }

    pub fn compose(&self, other: &LayerTransform) -> LayerTransform {
        let rad = self.rotation_deg * std::f32::consts::PI / 180.0;
        let cos_r = rad.cos();
        let sin_r = rad.sin();

        let new_tx = self.translate_x + other.translate_x * cos_r * self.scale_x
            - other.translate_y * sin_r * self.scale_y;
        let new_ty = self.translate_y
            + other.translate_x * sin_r * self.scale_x
            + other.translate_y * cos_r * self.scale_y;

        LayerTransform {
            translate_x: new_tx,
            translate_y: new_ty,
            scale_x: self.scale_x * other.scale_x,
            scale_y: self.scale_y * other.scale_y,
            rotation_deg: self.rotation_deg + other.rotation_deg,
            skew_x: self.skew_x + other.skew_x,
            skew_y: self.skew_y + other.skew_y,
        }
    }

    pub fn inverse_transform_point(&self, px: f32, py: f32) -> (f32, f32) {
        let dx = px - self.translate_x;
        let dy = py - self.translate_y;
        let rad = -self.rotation_deg * std::f32::consts::PI / 180.0;
        let cos_r = rad.cos();
        let sin_r = rad.sin();
        let sx = if self.scale_x.abs() > 0.0001 {
            1.0 / self.scale_x
        } else {
            0.0
        };
        let sy = if self.scale_y.abs() > 0.0001 {
            1.0 / self.scale_y
        } else {
            0.0
        };

        let rx = (dx * cos_r + dy * sin_r) * sx;
        let ry = (-dx * sin_r + dy * cos_r) * sy;
        (rx, ry)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[derive(Default)]
pub struct ZOrder(pub i32);


#[derive(Debug, Clone)]
pub struct CompositingLayer {
    pub id: u32,
    pub name: String,
    pub z_order: ZOrder,
    pub opacity: f32,
    pub visible: bool,
    pub blend_mode: CompositingBlendMode,
    pub transform: LayerTransform,
    pub width: u32,
    pub height: u32,
    pub pixel_data: Vec<u8>,
    pub locked: bool,
}

impl CompositingLayer {
    pub fn new(id: u32, name: &str, width: u32, height: u32) -> Self {
        CompositingLayer {
            id,
            name: name.to_string(),
            z_order: ZOrder(0),
            opacity: 1.0,
            visible: true,
            blend_mode: CompositingBlendMode::default(),
            transform: LayerTransform::default(),
            width,
            height,
            pixel_data: vec![0u8; width as usize * height as usize * 4],
            locked: false,
        }
    }

    pub fn with_z_order(mut self, z: i32) -> Self {
        self.z_order = ZOrder(z);
        self
    }

    pub fn with_opacity(mut self, opacity: f32) -> Self {
        self.opacity = opacity.clamp(0.0, 1.0);
        self
    }

    pub fn with_blend_mode(mut self, mode: CompositingBlendMode) -> Self {
        self.blend_mode = mode;
        self
    }

    pub fn with_transform(mut self, transform: LayerTransform) -> Self {
        self.transform = transform;
        self
    }

    pub fn set_opacity(&mut self, opacity: f32) {
        if !self.locked {
            self.opacity = opacity.clamp(0.0, 1.0);
        }
    }

    pub fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }

    pub fn set_z_order(&mut self, z: i32) {
        if !self.locked {
            self.z_order = ZOrder(z);
        }
    }

    pub fn clear(&mut self) {
        if !self.locked {
            self.pixel_data.fill(0);
        }
    }

    pub fn fill_solid(&mut self, r: u8, g: u8, b: u8, a: u8) {
        if self.locked {
            return;
        }
        for chunk in self.pixel_data.as_chunks_mut::<4>().0 {
            chunk[0] = r;
            chunk[1] = g;
            chunk[2] = b;
            chunk[3] = a;
        }
    }

    pub fn pixel_count(&self) -> usize {
        self.width as usize * self.height as usize
    }

    pub fn is_empty(&self) -> bool {
        self.pixel_data.iter().all(|&b| b == 0)
    }

    pub fn lock(&mut self) {
        self.locked = true;
    }

    pub fn unlock(&mut self) {
        self.locked = false;
    }

    pub fn bounds(&self) -> (f32, f32, f32, f32) {
        let w = self.width as f32 * self.transform.scale_x;
        let h = self.height as f32 * self.transform.scale_y;
        (
            self.transform.translate_x,
            self.transform.translate_y,
            w.abs(),
            h.abs(),
        )
    }
}

#[derive(Debug, Clone)]
pub struct Compositor {
    pub layers: Vec<CompositingLayer>,
    pub output_width: u32,
    pub output_height: u32,
    pub output_buffer: Vec<u8>,
    pub next_layer_id: u32,
    pub compositing_count: u64,
}

impl Default for Compositor {
    fn default() -> Self {
        Compositor {
            layers: Vec::new(),
            output_width: 800,
            output_height: 600,
            output_buffer: vec![0u8; 800 * 600 * 4],
            next_layer_id: 0,
            compositing_count: 0,
        }
    }
}

impl Compositor {
    pub fn new(width: u32, height: u32) -> Self {
        Compositor {
            output_width: width,
            output_height: height,
            output_buffer: vec![0u8; width as usize * height as usize * 4],
            ..Default::default()
        }
    }

    pub fn resize_output(&mut self, width: u32, height: u32) {
        self.output_width = width;
        self.output_height = height;
        self.output_buffer
            .resize(width as usize * height as usize * 4, 0);
        self.clear_output();
    }

    pub fn clear_output(&mut self) {
        self.output_buffer.fill(0);
    }

    pub fn add_layer(&mut self, layer: CompositingLayer) -> u32 {
        let id = self.next_layer_id;
        self.next_layer_id += 1;
        let mut layer = layer;
        layer.id = id;
        self.layers.push(layer);
        id
    }

    pub fn create_layer(&mut self, name: &str, width: u32, height: u32) -> u32 {
        let id = self.next_layer_id;
        let layer = CompositingLayer::new(id, name, width, height);
        self.add_layer(layer)
    }

    pub fn remove_layer(&mut self, id: u32) -> bool {
        if let Some(pos) = self.layers.iter().position(|l| l.id == id) {
            self.layers.remove(pos);
            true
        } else {
            false
        }
    }

    pub fn get_layer(&self, id: u32) -> Option<&CompositingLayer> {
        self.layers.iter().find(|l| l.id == id)
    }

    pub fn get_layer_mut(&mut self, id: u32) -> Option<&mut CompositingLayer> {
        self.layers.iter_mut().find(|l| l.id == id)
    }

    pub fn layer_count(&self) -> usize {
        self.layers.len()
    }

    pub fn visible_layers(&self) -> Vec<&CompositingLayer> {
        self.layers
            .iter()
            .filter(|l| l.visible && l.opacity > 0.0)
            .collect()
    }

    fn sort_layers(&mut self) {
        self.layers.sort_by_key(|l| l.z_order);
    }

    fn blend_pixel(dst: &mut [u8], src: &[u8], opacity: f32, mode: CompositingBlendMode) {
        let sa = src[3] as f32 / 255.0 * opacity;
        if sa <= 0.0 {
            return;
        }
        match mode {
            CompositingBlendMode::Normal | CompositingBlendMode::Additive => {
                let da = dst[3] as f32 / 255.0;
                let out_a = sa + da * (1.0 - sa);
                if out_a > 0.0 {
                    for c in 0..3 {
                        let sc = src[c] as f32;
                        let dc = dst[c] as f32;
                        let blended = if mode == CompositingBlendMode::Additive {
                            (sc * sa + dc * da).min(255.0)
                        } else {
                            (sc * sa + dc * da * (1.0 - sa)) / out_a
                        };
                        dst[c] = blended as u8;
                    }
                    dst[3] = (out_a * 255.0) as u8;
                }
            }
            CompositingBlendMode::Multiply => {
                for c in 0..3 {
                    let s = src[c] as f32 / 255.0;
                    let d = dst[c] as f32 / 255.0;
                    let blended = s * d;
                    dst[c] = (blended * 255.0 * sa + dst[c] as f32 * (1.0 - sa)) as u8;
                }
                dst[3] = (sa * 255.0 + dst[3] as f32 * (1.0 - sa)).min(255.0) as u8;
            }
            CompositingBlendMode::Screen => {
                for c in 0..3 {
                    let s = src[c] as f32 / 255.0;
                    let d = dst[c] as f32 / 255.0;
                    let blended = 1.0 - (1.0 - s) * (1.0 - d);
                    dst[c] = (blended * 255.0 * sa + dst[c] as f32 * (1.0 - sa)) as u8;
                }
                dst[3] = (sa * 255.0 + dst[3] as f32 * (1.0 - sa)).min(255.0) as u8;
            }
            CompositingBlendMode::Overlay => {
                for c in 0..3 {
                    let s = src[c] as f32 / 255.0;
                    let d = dst[c] as f32 / 255.0;
                    let blended = if d < 0.5 {
                        2.0 * s * d
                    } else {
                        1.0 - 2.0 * (1.0 - s) * (1.0 - d)
                    };
                    dst[c] = (blended * 255.0 * sa + dst[c] as f32 * (1.0 - sa)) as u8;
                }
                dst[3] = (sa * 255.0 + dst[3] as f32 * (1.0 - sa)).min(255.0) as u8;
            }
        }
    }

    pub fn composite(&mut self) {
        self.sort_layers();
        self.clear_output();

        let out_w = self.output_width as usize;
        let out_h = self.output_height as usize;

        for layer in &self.layers {
            if !layer.visible || layer.opacity <= 0.0 || layer.pixel_data.is_empty() {
                continue;
            }

            let layer_w = layer.width as usize;
            let layer_h = layer.height as usize;
            let tx = layer.transform.translate_x as i32;
            let ty = layer.transform.translate_y as i32;

            for ly in 0..layer_h {
                let out_y = ly as i32 + ty;
                if out_y < 0 || out_y >= out_h as i32 {
                    continue;
                }
                for lx in 0..layer_w {
                    let out_x = lx as i32 + tx;
                    if out_x < 0 || out_x >= out_w as i32 {
                        continue;
                    }
                    let src_idx = (ly * layer_w + lx) * 4;
                    let dst_idx = (out_y as usize * out_w + out_x as usize) * 4;

                    if src_idx + 4 <= layer.pixel_data.len()
                        && dst_idx + 4 <= self.output_buffer.len()
                    {
                        let src = layer.pixel_data[src_idx..src_idx + 4].to_vec();
                        Self::blend_pixel(
                            &mut self.output_buffer[dst_idx..dst_idx + 4],
                            &src,
                            layer.opacity,
                            layer.blend_mode,
                        );
                    }
                }
            }
        }
        self.compositing_count += 1;
    }

    pub fn output_pixel(&self, x: u32, y: u32) -> Option<&[u8]> {
        if x >= self.output_width || y >= self.output_height {
            return None;
        }
        let idx = (y as usize * self.output_width as usize + x as usize) * 4;
        Some(&self.output_buffer[idx..idx + 4])
    }

    pub fn output_is_empty(&self) -> bool {
        self.output_buffer.iter().all(|&b| b == 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_layer_transform_default() {
        let t = LayerTransform::default();
        assert_eq!(t.translate_x, 0.0);
        assert_eq!(t.scale_x, 1.0);
        assert_eq!(t.rotation_deg, 0.0);
    }

    #[test]
    fn test_layer_transform_translate() {
        let t = LayerTransform::translate(10.0, 20.0);
        assert_eq!(t.translate_x, 10.0);
        assert_eq!(t.translate_y, 20.0);
        assert_eq!(t.scale_x, 1.0);
    }

    #[test]
    fn test_layer_transform_compose() {
        let a = LayerTransform::translate(10.0, 0.0);
        let b = LayerTransform::translate(0.0, 5.0);
        let c = a.compose(&b);
        assert!((c.translate_x - 10.0).abs() < 0.01);
        assert!((c.translate_y - 5.0).abs() < 0.01);
    }

    #[test]
    fn test_layer_transform_inverse() {
        let t = LayerTransform::translate(100.0, 50.0);
        let (ix, iy) = t.inverse_transform_point(150.0, 100.0);
        assert!((ix - 50.0).abs() < 0.01);
        assert!((iy - 50.0).abs() < 0.01);
    }

    #[test]
    fn test_compositing_layer() {
        let mut layer = CompositingLayer::new(0, "test", 100, 100)
            .with_z_order(5)
            .with_opacity(0.8);

        assert_eq!(layer.id, 0);
        assert_eq!(layer.name, "test");
        assert_eq!(layer.z_order, ZOrder(5));
        assert_eq!(layer.opacity, 0.8);
        assert_eq!(layer.pixel_count(), 10000);

        layer.set_opacity(0.5);
        assert_eq!(layer.opacity, 0.5);

        layer.lock();
        layer.set_opacity(1.0);
        assert_eq!(layer.opacity, 0.5);

        layer.unlock();
        layer.set_opacity(1.0);
        assert_eq!(layer.opacity, 1.0);
    }

    #[test]
    fn test_layer_fill() {
        let mut layer = CompositingLayer::new(0, "bg", 2, 2);
        layer.fill_solid(255, 0, 0, 255);
        assert_eq!(layer.pixel_data[0], 255);
        assert_eq!(layer.pixel_data[1], 0);
        assert_eq!(layer.pixel_data[2], 0);
        assert_eq!(layer.pixel_data[3], 255);
        assert!(!layer.is_empty());

        layer.lock();
        layer.fill_solid(0, 0, 0, 0);
        assert_eq!(layer.pixel_data[0], 255);
        layer.unlock();
    }

    #[test]
    fn test_layer_bounds() {
        let layer = CompositingLayer::new(0, "test", 100, 50)
            .with_transform(LayerTransform::translate(10.0, 20.0));
        let (x, y, w, h) = layer.bounds();
        assert_eq!(x, 10.0);
        assert_eq!(y, 20.0);
        assert_eq!(w, 100.0);
        assert_eq!(h, 50.0);
    }

    #[test]
    fn test_compositor() {
        let mut comp = Compositor::new(10, 10);
        assert_eq!(comp.output_width, 10);
        assert_eq!(comp.output_height, 10);
        assert_eq!(comp.layer_count(), 0);

        let id = comp.create_layer("bg", 10, 10);
        assert_eq!(comp.layer_count(), 1);
        assert!(comp.get_layer(id).is_some());

        comp.remove_layer(id);
        assert_eq!(comp.layer_count(), 0);
    }

    #[test]
    fn test_compositor_composite() {
        let mut comp = Compositor::new(4, 4);

        let mut layer = CompositingLayer::new(0, "red", 4, 4).with_z_order(1);
        layer.fill_solid(255, 0, 0, 255);
        comp.add_layer(layer);

        let mut layer2 = CompositingLayer::new(1, "green", 4, 4).with_z_order(2);
        layer2.fill_solid(0, 255, 0, 255);
        comp.add_layer(layer2);

        comp.composite();
        assert!(!comp.output_is_empty());
        assert_eq!(comp.compositing_count, 1);
    }

    #[test]
    fn test_compositor_output_pixel() {
        let comp = Compositor::new(4, 4);
        assert!(comp.output_pixel(0, 0).is_some());
        assert!(comp.output_pixel(10, 10).is_none());
    }

    #[test]
    fn test_compositor_resize() {
        let mut comp = Compositor::new(10, 10);
        comp.resize_output(20, 20);
        assert_eq!(comp.output_width, 20);
        assert_eq!(comp.output_height, 20);
        assert!(comp.output_is_empty());
    }

    #[test]
    fn test_compositor_visible_layers() {
        let mut comp = Compositor::new(10, 10);
        let _id1 = comp.create_layer("a", 10, 10);
        let id2 = comp.create_layer("b", 10, 10);

        comp.get_layer_mut(id2).unwrap().set_visible(false);

        let visible = comp.visible_layers();
        assert_eq!(visible.len(), 1);
        assert_eq!(visible[0].name, "a");
    }

    #[test]
    fn test_blend_modes() {
        let mut comp = Compositor::new(2, 2);

        let mut layer =
            CompositingLayer::new(0, "test", 2, 2).with_blend_mode(CompositingBlendMode::Screen);
        layer.fill_solid(128, 128, 128, 255);
        comp.add_layer(layer);
        comp.composite();

        assert!(!comp.output_is_empty());
    }

    #[test]
    fn test_z_order_sorting() {
        let mut comp = Compositor::new(2, 2);
        comp.add_layer(CompositingLayer::new(0, "top", 2, 2).with_z_order(10));
        comp.add_layer(CompositingLayer::new(1, "bottom", 2, 2).with_z_order(-10));

        comp.sort_layers();
        assert_eq!(comp.layers[0].name, "bottom");
        assert_eq!(comp.layers[1].name, "top");
    }

    #[test]
    fn test_z_order_comparison() {
        assert!(ZOrder(-1) < ZOrder(0));
        assert!(ZOrder(0) < ZOrder(1));
        assert!(ZOrder(5) > ZOrder(3));
    }
}
