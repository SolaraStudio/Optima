use crate::css::value::Value;

#[derive(Debug, Clone)]
pub enum TransformFunction {
    Translate(f32, f32),
    Scale(f32, f32),
    Rotate(f32),
    Skew(f32, f32),
    Matrix([f32; 6]),
    TranslateX(f32),
    TranslateY(f32),
    ScaleX(f32),
    ScaleY(f32),
}

pub struct Transform {
    pub functions: Vec<TransformFunction>,
}

impl Transform {
    pub fn new() -> Self {
        Transform {
            functions: Vec::new(),
        }
    }

    pub fn from_value(value: &Value) -> Option<Self> {
        // Parse transform functions
        None
    }
}
