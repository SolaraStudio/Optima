#[derive(Debug, Clone, Copy)]
pub struct Transform {
    pub translate_x: f32,
    pub translate_y: f32,
    pub scale_x: f32,
    pub scale_y: f32,
    pub rotation: f32,
    pub skew_x: f32,
    pub skew_y: f32,
}

impl Transform {
    pub fn identity() -> Self {
        Self {
            translate_x: 0.0,
            translate_y: 0.0,
            scale_x: 1.0,
            scale_y: 1.0,
            rotation: 0.0,
            skew_x: 0.0,
            skew_y: 0.0,
        }
    }

    pub fn translate(x: f32, y: f32) -> Self {
        let mut t = Self::identity();
        t.translate_x = x;
        t.translate_y = y;
        t
    }

    pub fn scale(x: f32, y: f32) -> Self {
        let mut t = Self::identity();
        t.scale_x = x;
        t.scale_y = y;
        t
    }

    pub fn rotate(angle: f32) -> Self {
        let mut t = Self::identity();
        t.rotation = angle;
        t
    }

    pub fn apply(&self, x: f32, y: f32) -> (f32, f32) {
        let cos = self.rotation.cos();
        let sin = self.rotation.sin();
        let rx = x * self.scale_x * cos - y * self.scale_y * sin + self.translate_x;
        let ry = x * self.scale_x * sin + y * self.scale_y * cos + self.translate_y;
        (rx, ry)
    }
}
