use vello::kurbo::Rect;

pub struct ClipRegion {
    pub rect: Rect,
}

impl ClipRegion {
    pub fn new(x: f64, y: f64, w: f64, h: f64) -> Self {
        Self {
            rect: Rect::from_origin_size((x, y), (w, h)),
        }
    }

    pub fn intersect(&self, other: &ClipRegion) -> ClipRegion {
        let x1 = self.rect.x0.max(other.rect.x0);
        let y1 = self.rect.y0.max(other.rect.y0);
        let x2 = self.rect.x1.min(other.rect.x1);
        let y2 = self.rect.y1.min(other.rect.y1);
        if x1 < x2 && y1 < y2 {
            ClipRegion::new(x1, y1, x2 - x1, y2 - y1)
        } else {
            ClipRegion::new(0.0, 0.0, 0.0, 0.0)
        }
    }

    pub fn contains(&self, x: f64, y: f64) -> bool {
        x >= self.rect.x0 && x <= self.rect.x1 && y >= self.rect.y0 && y <= self.rect.y1
    }
}
