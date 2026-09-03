pub struct RenderBackend {
    pub device: Option<wgpu::Device>,
    pub queue: Option<wgpu::Queue>,
    pub surface: Option<wgpu::Surface<'static>>,
}

impl RenderBackend {
    pub fn new() -> Self {
        RenderBackend {
            device: None,
            queue: None,
            surface: None,
        }
    }

    pub fn initialize(&mut self, _width: u32, _height: u32) -> Result<(), String> {
        let instance = wgpu::Instance::default();
        let adapter = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            compatible_surface: None,
            force_fallback_adapter: false,
        }))
        .map_err(|e| format!("Failed to request adapter: {}", e))?;
        let (device, queue) = pollster::block_on(adapter.request_device(&wgpu::DeviceDescriptor {
            label: Some("Optima Device"),
            required_features: wgpu::Features::empty(),
            required_limits: wgpu::Limits::default(),
            ..Default::default()
        }))
        .map_err(|e| format!("Failed to create device: {}", e))?;
        self.device = Some(device);
        self.queue = Some(queue);
        Ok(())
    }

    pub fn is_initialized(&self) -> bool {
        self.device.is_some()
    }
}
