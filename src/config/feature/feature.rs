#[derive(Debug, Clone, Default)]
pub struct FeatureFlags {
    pub enable_gpu: bool,
    pub enable_webgl: bool,
    pub enable_webaudio: bool,
    pub enable_webvideo: bool,
    pub enable_websocket: bool,
    pub enable_fetch: bool,
    pub enable_service_worker: bool,
}

impl FeatureFlags {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn gpu(mut self, v: bool) -> Self {
        self.enable_gpu = v;
        self
    }
    pub fn webgl(mut self, v: bool) -> Self {
        self.enable_webgl = v;
        self
    }
}
