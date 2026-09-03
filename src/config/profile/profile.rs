#[derive(Debug, Clone, PartialEq)]
pub enum Profile {
    Debug,
    Release,
    Profile,
}

impl Default for Profile {
    fn default() -> Self { Profile::Release }
}

impl Profile {
    pub fn is_release(&self) -> bool { *self == Profile::Release }
    pub fn is_debug(&self) -> bool { *self == Profile::Debug }
}
