pub struct BoxModel {
    pub margin: f32,
    pub padding: f32,
    pub border: f32,
}

impl BoxModel {
    pub fn new() -> Self {
        Self {
            margin: 0.0,
            padding: 0.0,
            border: 0.0,
        }
    }
}
