pub struct JniBridge;

impl JniBridge {
    pub fn new() -> Self { JniBridge }
    pub fn version(&self) -> &str { env!("CARGO_PKG_VERSION") }
}
