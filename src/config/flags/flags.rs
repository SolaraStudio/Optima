use std::collections::HashSet;

#[derive(Debug, Clone, Default)]
pub struct Flags {
    pub set: HashSet<String>,
}

impl Flags {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set(&mut self, flag: &str) {
        self.set.insert(flag.to_string());
    }

    pub fn is_set(&self, flag: &str) -> bool {
        self.set.contains(flag)
    }

    pub fn clear(&mut self, flag: &str) {
        self.set.remove(flag);
    }
}
