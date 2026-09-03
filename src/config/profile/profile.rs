#[derive(Debug, Clone, PartialEq)]
#[derive(Default)]
pub enum Profile {
    Debug,
    #[default]
    Release,
    Profile,
}


impl Profile {
    pub fn is_release(&self) -> bool {
        *self == Profile::Release
    }
    pub fn is_debug(&self) -> bool {
        *self == Profile::Debug
    }
}
