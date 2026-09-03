use crate::css::declaration::Declaration;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Keyframes {
    pub name: String,
    pub keyframes: Vec<Keyframe>,
}

#[derive(Debug, Clone)]
pub struct Keyframe {
    pub keys: Vec<f32>,
    pub declarations: Vec<Declaration>,
}

impl Keyframes {
    pub fn new(name: &str) -> Self {
        Keyframes {
            name: name.to_string(),
            keyframes: Vec::new(),
        }
    }

    pub fn add_keyframe(&mut self, keys: Vec<f32>, declarations: Vec<Declaration>) {
        self.keyframes.push(Keyframe { keys, declarations });
    }

    pub fn get_declarations_at(&self, progress: f32) -> Vec<Declaration> {
        let mut result = Vec::new();
        for keyframe in &self.keyframes {
            for &key in &keyframe.keys {
                if key <= progress {
                    result.extend(keyframe.declarations.clone());
                    break;
                }
            }
        }
        result
    }
}
