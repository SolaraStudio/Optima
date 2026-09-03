#[derive(Debug, Clone)]
pub struct DebugConfig {
    pub enable_logging: bool,
    pub enable_devtools: bool,
    pub log_level: LogLevel,
    pub show_layout_bounds: bool,
    pub show_paint_bounds: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum LogLevel {
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

impl Default for DebugConfig {
    fn default() -> Self {
        DebugConfig {
            enable_logging: cfg!(debug_assertions),
            enable_devtools: cfg!(debug_assertions),
            log_level: LogLevel::Info,
            show_layout_bounds: false,
            show_paint_bounds: false,
        }
    }
}

impl DebugConfig {
    pub fn new() -> Self { Self::default() }
    pub fn with_logging(mut self, enabled: bool) -> Self { self.enable_logging = enabled; self }
    pub fn with_devtools(mut self, enabled: bool) -> Self { self.enable_devtools = enabled; self }
}
