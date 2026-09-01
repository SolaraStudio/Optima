use super::VideoFrame;
use vello::SceneBuilder;
use vello::kurbo::Rect;
use vello::peniko::{Color, Fill};
use wgpu::{Device, Texture, TextureView, Sampler};

pub struct MediaRenderer {
    pub texture: Option<Texture>,
    pub texture_view: Option<TextureView>,
    pub sampler: Option<Sampler>,
    pub width: u32,
    pub height: u32,
}

impl MediaRenderer {
    pub fn new() -> Self {
        Self {
            texture: None,
            texture_view: None,
            sampler: None,
            width: 0,
            height: 0,
        }
    }

    pub fn update_frame(&mut self, device: &Device, frame: &VideoFrame) {
        if self.width != frame.width || self.height != frame.height {
            self.width = frame.width;
            self.height = frame.height;
            let size = wgpu::Extent3d {
                width: frame.width,
                height: frame.height,
                depth_or_array_layers: 1,
            };
            let texture = device.create_texture(&wgpu::TextureDescriptor {
                label: Some("Video texture"),
                size,
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: wgpu::TextureFormat::Rgba8UnormSrgb,
                usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
                view_formats: &[],
            });
            self.texture = Some(texture);
            let view = self.texture.as_ref().unwrap().create_view(&wgpu::TextureViewDescriptor::default());
            self.texture_view = Some(view);
            self.sampler = Some(device.create_sampler(&wgpu::SamplerDescriptor {
                label: Some("Video sampler"),
                address_mode_u: wgpu::AddressMode::ClampToEdge,
                address_mode_v: wgpu::AddressMode::ClampToEdge,
                address_mode_w: wgpu::AddressMode::ClampToEdge,
                mag_filter: wgpu::FilterMode::Linear,
                min_filter: wgpu::FilterMode::Linear,
                mipmap_filter: wgpu::FilterMode::Linear,
                ..Default::default()
            }));
        }

        if let Some(texture) = &self.texture {
            let queue = device.queue();
            queue.write_texture(
                texture.as_image_copy(),
                &frame.data,
                wgpu::ImageDataLayout {
                    offset: 0,
                    bytes_per_row: Some(4 * frame.width),
                    rows_per_image: Some(frame.height),
                },
                wgpu::Extent3d {
                    width: frame.width,
                    height: frame.height,
                    depth_or_array_layers: 1,
                },
            );
        }
    }

    pub fn draw(&self, scene: &mut SceneBuilder, x: f64, y: f64, width: f64, height: f64) {
        if self.texture_view.is_some() && self.sampler.is_some() {
            // Draw a colored rectangle as a placeholder
            // In full implementation, you would draw the texture using Vello's image API
            let rect = Rect::from_origin_size((x, y), (width, height));
            scene.fill(
                Fill::NonZero,
                rect,
                &Color::new(0.2, 0.3, 0.8, 1.0),
                None,
            );
        }
    }
}

impl Default for MediaRenderer {
    fn default() -> Self {
        Self::new()
    }
}
