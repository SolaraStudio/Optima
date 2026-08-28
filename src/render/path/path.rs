use vello::kurbo::BezPath;

pub struct Path {
    pub inner: BezPath,
}

impl Path {
    pub fn new() -> Self {
        Self {
            inner: BezPath::new(),
        }
    }

    pub fn move_to(&mut self, x: f64, y: f64) {
        self.inner.move_to((x, y));
    }

    pub fn line_to(&mut self, x: f64, y: f64) {
        self.inner.line_to((x, y));
    }

    pub fn quadratic_to(&mut self, cx: f64, cy: f64, x: f64, y: f64) {
        self.inner.quad_to((cx, cy), (x, y));
    }

    pub fn cubic_to(&mut self, c1x: f64, c1y: f64, c2x: f64, c2y: f64, x: f64, y: f64) {
        self.inner.curve_to((c1x, c1y), (c2x, c2y), (x, y));
    }

    pub fn close(&mut self) {
        self.inner.close_path();
    }

    pub fn build(self) -> BezPath {
        self.inner
    }
}
