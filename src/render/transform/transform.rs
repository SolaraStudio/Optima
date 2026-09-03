#[derive(Debug, Clone)]
pub struct RenderTransform {
    pub matrix: [f32; 6],
}

impl Default for RenderTransform {
    fn default() -> Self { RenderTransform { matrix: [1.0, 0.0, 0.0, 1.0, 0.0, 0.0] } }
}

impl RenderTransform {
    pub fn identity() -> Self { Self::default() }

    pub fn translate(tx: f32, ty: f32) -> Self {
        RenderTransform { matrix: [1.0, 0.0, 0.0, 1.0, tx, ty] }
    }

    pub fn scale(sx: f32, sy: f32) -> Self {
        RenderTransform { matrix: [sx, 0.0, 0.0, sy, 0.0, 0.0] }
    }

    pub fn rotate(angle_rad: f32) -> Self {
        let c = angle_rad.cos(); let s = angle_rad.sin();
        RenderTransform { matrix: [c, s, -s, c, 0.0, 0.0] }
    }

    pub fn skew(sx: f32, sy: f32) -> Self {
        RenderTransform { matrix: [1.0, sy.tan(), sx.tan(), 1.0, 0.0, 0.0] }
    }

    pub fn multiply(&self, other: &RenderTransform) -> RenderTransform {
        let a = &self.matrix; let b = &other.matrix;
        RenderTransform { matrix: [
            a[0]*b[0] + a[2]*b[1], a[1]*b[0] + a[3]*b[1],
            a[0]*b[2] + a[2]*b[3], a[1]*b[2] + a[3]*b[3],
            a[0]*b[4] + a[2]*b[5] + a[4], a[1]*b[4] + a[3]*b[5] + a[5],
        ]}
    }

    pub fn transform_point(&self, x: f32, y: f32) -> (f32, f32) {
        let m = &self.matrix;
        (m[0]*x + m[2]*y + m[4], m[1]*x + m[3]*y + m[5])
    }

    pub fn determinant(&self) -> f32 {
        self.matrix[0]*self.matrix[3] - self.matrix[1]*self.matrix[2]
    }

    pub fn inverse(&self) -> Option<RenderTransform> {
        let det = self.determinant();
        if det.abs() < 1e-10 { return None; }
        let inv_det = 1.0 / det;
        let m = &self.matrix;
        Some(RenderTransform { matrix: [
            m[3]*inv_det, -m[1]*inv_det, -m[2]*inv_det, m[0]*inv_det,
            (m[2]*m[5] - m[3]*m[4])*inv_det, (m[1]*m[4] - m[0]*m[5])*inv_det,
        ]})
    }
}
