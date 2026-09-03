pub const ENGINE_NAME: &str = "Optima";
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const MAJOR: u32 = 0;
pub const MINOR: u32 = 150;
pub const PATCH: u32 = 10;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Version {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
}

impl Version {
    pub fn new(major: u32, minor: u32, patch: u32) -> Self {
        Version {
            major,
            minor,
            patch,
        }
    }

    pub fn current() -> Self {
        Version::new(MAJOR, MINOR, PATCH)
    }

    pub fn to_string(&self) -> String {
        format!("{}.{}.{}", self.major, self.minor, self.patch)
    }

    pub fn is_compatible(&self, required: &Version) -> bool {
        self.major == required.major
            && (self.minor > required.minor
                || (self.minor == required.minor && self.patch >= required.patch))
    }
}
