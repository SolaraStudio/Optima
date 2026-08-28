use vello::kurbo::{Rect, Circle, RoundedRect, BezPath, Line};

pub struct Shapes;

impl Shapes {
    pub fn rect(x: f64, y: f64, w: f64, h: f64) -> Rect {
        Rect::from_origin_size((x, y), (w, h))
    }

    pub fn circle(cx: f64, cy: f64, r: f64) -> Circle {
        Circle::new((cx, cy), r)
    }

    pub fn rounded_rect(x: f64, y: f64, w: f64, h: f64, radius: f64) -> RoundedRect {
        RoundedRect::from_rect(Rect::from_origin_size((x, y), (w, h)), radius)
    }

    pub fn line(x1: f64, y1: f64, x2: f64, y2: f64) -> Line {
        Line::new((x1, y1), (x2, y2))
    }

    pub fn path(points: &[(f64, f64)]) -> BezPath {
        let mut path = BezPath::new();
        for (i, &(x, y)) in points.iter().enumerate() {
            if i == 0 {
                path.move_to((x, y));
            } else {
                path.line_to((x, y));
            }
        }
        path
    }
}
