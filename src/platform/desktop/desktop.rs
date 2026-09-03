pub struct DesktopPlatform {
    pub window_title: String,
    pub width: u32,
    pub height: u32,
    pub resizable: bool,
}

impl Default for DesktopPlatform {
    fn default() -> Self {
        DesktopPlatform {
            window_title: "Optima".to_string(),
            width: 800,
            height: 600,
            resizable: true,
        }
    }
}

impl DesktopPlatform {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_title(mut self, title: &str) -> Self {
        self.window_title = title.to_string();
        self
    }
    pub fn with_size(mut self, w: u32, h: u32) -> Self {
        self.width = w;
        self.height = h;
        self
    }
}
