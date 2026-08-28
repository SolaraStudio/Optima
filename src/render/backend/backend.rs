use wgpu::{Device, Queue, Surface, SurfaceConfiguration, TextureFormat};
use vello::Renderer;

pub struct RenderBackend {
    pub device: Device,
    pub queue: Queue,
    pub surface: Surface,
    pub config: SurfaceConfiguration,
    pub renderer: Renderer,
}

impl RenderBackend {
    pub fn new(device: Device, queue: Queue, surface: Surface, config: SurfaceConfiguration) -> Self {
        let renderer = Renderer::new(&device, vello::RendererOptions::default()).unwrap();
        Self {
            device,
            queue,
            surface,
            config,
            renderer,
        }
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.config.width = width;
        self.config.height = height;
        self.surface.configure(&self.device, &self.config);
    }

    pub fn get_surface_format(&self) -> TextureFormat {
        self.config.format
    }
}
