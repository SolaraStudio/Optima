use crate::render::effect::Effect;

pub struct FilterChain {
    pub effects: Vec<Effect>,
}

impl Default for FilterChain {
    fn default() -> Self {
        FilterChain {
            effects: Vec::new(),
        }
    }
}

impl FilterChain {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn add(&mut self, effect: Effect) {
        self.effects.push(effect);
    }
    pub fn clear(&mut self) {
        self.effects.clear();
    }
    pub fn is_empty(&self) -> bool {
        self.effects.is_empty()
    }
}
