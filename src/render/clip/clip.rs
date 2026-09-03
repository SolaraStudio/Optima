use crate::render::path::Path;

#[derive(Debug, Clone, PartialEq)]
pub enum ClipRule {
    NonZero,
    EvenOdd,
}

#[derive(Debug, Clone)]
pub struct ClipRegion {
    pub path: Path,
    pub rule: ClipRule,
}

impl ClipRegion {
    pub fn new(path: Path) -> Self {
        ClipRegion {
            path,
            rule: ClipRule::NonZero,
        }
    }
    pub fn with_rule(mut self, rule: ClipRule) -> Self {
        self.rule = rule;
        self
    }

    pub fn rect(x: f32, y: f32, w: f32, h: f32) -> Self {
        ClipRegion::new(Path::rect(x, y, w, h))
    }

    pub fn contains_point(&self, x: f32, y: f32) -> bool {
        let bounds = self.path.bounds();
        x >= bounds.0 && x <= bounds.0 + bounds.2 && y >= bounds.1 && y <= bounds.1 + bounds.3
    }
}
