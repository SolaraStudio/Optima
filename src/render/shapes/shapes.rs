use crate::render::path::Path;

pub struct Shapes;

impl Shapes {
    pub fn rect(x: f32, y: f32, w: f32, h: f32) -> Path {
        Path::rect(x, y, w, h)
    }
    pub fn rounded_rect(x: f32, y: f32, w: f32, h: f32, _rx: f32, _ry: f32) -> Path {
        Path::rect(x, y, w, h)
    }
    pub fn circle(_cx: f32, _cy: f32, _r: f32) -> Path {
        Path::new()
    }
    pub fn ellipse(_cx: f32, _cy: f32, _rx: f32, _ry: f32) -> Path {
        Path::new()
    }
    pub fn line(x1: f32, y1: f32, x2: f32, y2: f32) -> Path {
        Path::new().move_to(x1, y1).line_to(x2, y2)
    }
    pub fn polygon(points: &[(f32, f32)]) -> Path {
        let mut path = Path::new();
        if let Some(&(x, y)) = points.first() {
            path = path.move_to(x, y);
        }
        for &(x, y) in &points[1..] {
            path = path.line_to(x, y);
        }
        path.close()
    }
}
