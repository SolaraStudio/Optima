use crate::render::path::Path;

#[derive(Debug, Clone, PartialEq)]
pub enum MaskType {
    Luminance,
    Alpha,
}

#[derive(Debug, Clone)]
pub struct Mask {
    pub path: Path,
    pub mask_type: MaskType,
    pub invert: bool,
}

impl Mask {
    pub fn new(path: Path) -> Self {
        Mask {
            path,
            mask_type: MaskType::Alpha,
            invert: false,
        }
    }

    pub fn with_luminance(path: Path) -> Self {
        Mask {
            path,
            mask_type: MaskType::Luminance,
            invert: false,
        }
    }

    pub fn inverted(mut self) -> Self {
        self.invert = true;
        self
    }
}
