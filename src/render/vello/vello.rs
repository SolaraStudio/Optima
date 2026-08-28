use jni::objects::JObject;
use vello::kurbo::{Rect, Vec2};
use vello::peniko::{Color, Fill};
use vello::{SceneBuilder, Renderer, RendererOptions};
use wgpu::{Device, Queue, Surface, SurfaceConfiguration, TextureFormat};
use wgpu::util::DeviceExt;

pub struct VelloRenderer {
    device: Device,
    queue: Queue,
    surface: Surface,
    config: SurfaceConfiguration,
    renderer: Renderer,
}

impl VelloRenderer {
    pub fn new(surface: JObject) -> Self {
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor::default());
        let surface = unsafe { instance.create_surface_from_android(surface) }.unwrap();
        let adapter = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        })).unwrap();
        let (device, queue) = pollster::block_on(adapter.request_device(&wgpu::DeviceDescriptor {
            label: None,
            required_features: wgpu::Features::empty(),
            required_limits: wgpu::Limits::default(),
        }, None)).unwrap();

        let size = surface.get_current_texture().unwrap().texture.size();
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: TextureFormat::Bgra8UnormSrgb,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
            desired_maximum_frame_latency: 2,
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
            view_formats: vec![],
        };
        surface.configure(&device, &config);

        let renderer = Renderer::new(&device, RendererOptions::default()).unwrap();

        Self {
            device,
            queue,
            surface,
            config,
            renderer,
        }
    }

    pub fn render(&mut self) {
        let surface_texture = self.surface.get_current_texture().unwrap();
        let view = surface_texture.texture.create_view(&wgpu::TextureViewDescriptor::default());

        let mut scene = SceneBuilder::new();
        scene.fill(
            Fill::NonZero,
            Rect::from_origin_size((0.0, 0.0), (400.0, 400.0)),
            &Color::new(0.5, 0.2, 0.8, 1.0),
            None,
        );
        let scene = scene.build();

        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor::default());
        let mut renderer = &mut self.renderer;
        let (render_encoder, resolve_targets) = renderer.render_to_surface(
            &self.device,
            &self.queue,
            &scene,
            &view,
            &self.config,
            &mut encoder,
        ).unwrap();

        self.queue.submit(Some(encoder.finish()));
        surface_texture.present();
    }
}
