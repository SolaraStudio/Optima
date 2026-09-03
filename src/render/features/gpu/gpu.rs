use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[derive(Default)]
pub enum GpuBackend {
    #[default]
    Vulkan,
    Metal,
    Dx12,
    Gl,
    WebGpu,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[derive(Default)]
pub enum PowerPreference {
    LowPower,
    HighPerformance,
    #[default]
    Default,
}


#[derive(Debug, Clone)]
pub struct GpuLimits {
    pub max_texture_dimension: u32,
    pub max_texture_dimension_3d: u32,
    pub max_texture_array_layers: u32,
    pub max_bind_groups: u32,
    pub max_uniform_buffer_binding_size: u64,
    pub max_storage_buffer_binding_size: u64,
    pub max_push_constant_size: u32,
    pub max_buffer_size: u64,
}

impl Default for GpuLimits {
    fn default() -> Self {
        GpuLimits {
            max_texture_dimension: 8192,
            max_texture_dimension_3d: 2048,
            max_texture_array_layers: 256,
            max_bind_groups: 4,
            max_uniform_buffer_binding_size: 65536,
            max_storage_buffer_binding_size: 128 * 1024 * 1024,
            max_push_constant_size: 256,
            max_buffer_size: 256 * 1024 * 1024,
        }
    }
}

#[derive(Debug, Clone)]
pub struct BindGroupEntry {
    pub binding: u32,
    pub name: String,
    pub visibility: ShaderStage,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[derive(Default)]
pub enum ShaderStage {
    Vertex,
    Fragment,
    Compute,
    #[default]
    VertexFragment,
}


#[derive(Debug, Clone)]
pub struct BindGroupLayout {
    pub entries: Vec<BindGroupEntry>,
    pub label: String,
}

impl BindGroupLayout {
    pub fn new(label: &str) -> Self {
        BindGroupLayout {
            entries: Vec::new(),
            label: label.to_string(),
        }
    }

    pub fn add_entry(&mut self, binding: u32, name: &str, visibility: ShaderStage) {
        self.entries.push(BindGroupEntry {
            binding,
            name: name.to_string(),
            visibility,
        });
    }

    pub fn entry_count(&self) -> usize {
        self.entries.len()
    }

    pub fn find_entry(&self, binding: u32) -> Option<&BindGroupEntry> {
        self.entries.iter().find(|e| e.binding == binding)
    }
}

#[derive(Debug, Clone)]
#[derive(Default)]
pub struct PipelineDescriptor {
    pub label: String,
    pub vertex_shader: Option<String>,
    pub fragment_shader: Option<String>,
    pub bind_group_layouts: Vec<BindGroupLayout>,
    pub push_constant_ranges: Vec<PushConstantRange>,
    pub pipeline_layout_hash: u64,
}

#[derive(Debug, Clone, Copy)]
pub struct PushConstantRange {
    pub offset: u32,
    pub size: u32,
    pub stage: ShaderStage,
}


impl PipelineDescriptor {
    pub fn new(label: &str) -> Self {
        PipelineDescriptor {
            label: label.to_string(),
            ..Default::default()
        }
    }

    pub fn with_vertex_shader(mut self, shader: &str) -> Self {
        self.vertex_shader = Some(shader.to_string());
        self
    }

    pub fn with_fragment_shader(mut self, shader: &str) -> Self {
        self.fragment_shader = Some(shader.to_string());
        self
    }

    pub fn add_bind_group_layout(&mut self, layout: BindGroupLayout) {
        self.bind_group_layouts.push(layout);
    }

    pub fn add_push_constant_range(&mut self, range: PushConstantRange) {
        self.push_constant_ranges.push(range);
    }

    pub fn is_complete(&self) -> bool {
        self.vertex_shader.is_some() && !self.bind_group_layouts.is_empty()
    }

    pub fn compute_layout_hash(&mut self) {
        let mut hash: u64 = 14695981039346656037;
        for layout in &self.bind_group_layouts {
            for entry in &layout.entries {
                hash ^= entry.binding as u64;
                hash = hash.wrapping_mul(1099511628211);
                hash ^= entry.visibility as u64;
                hash = hash.wrapping_mul(1099511628211);
            }
        }
        self.pipeline_layout_hash = hash;
    }
}

#[derive(Debug, Clone)]
pub struct GpuSurface {
    pub width: u32,
    pub height: u32,
    pub format: TextureFormat,
    pub vsync: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[derive(Default)]
pub enum TextureFormat {
    #[default]
    Bgra8Unorm,
    Rgba8Unorm,
    Rgba16Float,
    Bgra8UnormSrgb,
    Depth32Float,
}


impl Default for GpuSurface {
    fn default() -> Self {
        GpuSurface {
            width: 800,
            height: 600,
            format: TextureFormat::Bgra8Unorm,
            vsync: true,
        }
    }
}

impl GpuSurface {
    pub fn new(width: u32, height: u32) -> Self {
        GpuSurface {
            width,
            height,
            ..Default::default()
        }
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.width = width.max(1);
        self.height = height.max(1);
    }

    pub fn aspect_ratio(&self) -> f32 {
        self.width as f32 / self.height as f32
    }

    pub fn pixel_count(&self) -> u64 {
        self.width as u64 * self.height as u64
    }
}

#[derive(Debug, Clone)]
#[derive(Default)]
pub struct GpuPipeline {
    pub descriptor: PipelineDescriptor,
    pub surface: GpuSurface,
    pub initialized: bool,
    pub draw_calls: u64,
}


impl GpuPipeline {
    pub fn new(label: &str, width: u32, height: u32) -> Self {
        GpuPipeline {
            descriptor: PipelineDescriptor::new(label),
            surface: GpuSurface::new(width, height),
            initialized: false,
            draw_calls: 0,
        }
    }

    pub fn initialize(&mut self) -> Result<(), String> {
        if self.descriptor.is_complete() {
            self.initialized = true;
            self.draw_calls = 0;
            Ok(())
        } else {
            Err(
                "Pipeline descriptor is incomplete: missing shader or bind group layouts"
                    .to_string(),
            )
        }
    }

    pub fn begin_render_pass(&mut self) -> bool {
        if self.initialized {
            self.draw_calls = 0;
            true
        } else {
            false
        }
    }

    pub fn draw(&mut self, vertex_count: u32, instance_count: u32) -> bool {
        if self.initialized {
            self.draw_calls += 1;
            let _ = vertex_count;
            let _ = instance_count;
            true
        } else {
            false
        }
    }

    pub fn end_render_pass(&self) -> u64 {
        self.draw_calls
    }

    pub fn resize_surface(&mut self, width: u32, height: u32) {
        self.surface.resize(width, height);
    }
}

#[derive(Debug)]
#[derive(Default)]
pub struct GpuState {
    pub backend: GpuBackend,
    pub power_preference: PowerPreference,
    pub limits: GpuLimits,
    pub pipeline: GpuPipeline,
    pub bind_groups: HashMap<String, Vec<u32>>,
    pub frame_number: u64,
    pub total_vertices: u64,
}


impl GpuState {
    pub fn new(backend: GpuBackend) -> Self {
        GpuState {
            backend,
            ..Default::default()
        }
    }

    pub fn create_bind_group(&mut self, name: &str, bindings: Vec<u32>) {
        self.bind_groups.insert(name.to_string(), bindings);
    }

    pub fn get_bind_group(&self, name: &str) -> Option<&Vec<u32>> {
        self.bind_groups.get(name)
    }

    pub fn remove_bind_group(&mut self, name: &str) -> bool {
        self.bind_groups.remove(name).is_some()
    }

    pub fn begin_frame(&mut self) -> u64 {
        self.frame_number += 1;
        self.frame_number
    }

    pub fn end_frame(&mut self, vertices: u64) {
        self.total_vertices += vertices;
    }

    pub fn reset_stats(&mut self) {
        self.frame_number = 0;
        self.total_vertices = 0;
    }

    pub fn vertices_per_frame(&self) -> f64 {
        if self.frame_number > 0 {
            self.total_vertices as f64 / self.frame_number as f64
        } else {
            0.0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gpu_backend_default() {
        let backend = GpuBackend::default();
        assert_eq!(backend, GpuBackend::Vulkan);
    }

    #[test]
    fn test_power_preference_default() {
        let pref = PowerPreference::default();
        assert_eq!(pref, PowerPreference::Default);
    }

    #[test]
    fn test_gpu_limits_default() {
        let limits = GpuLimits::default();
        assert_eq!(limits.max_texture_dimension, 8192);
        assert_eq!(limits.max_bind_groups, 4);
        assert_eq!(limits.max_buffer_size, 256 * 1024 * 1024);
    }

    #[test]
    fn test_bind_group_layout() {
        let mut layout = BindGroupLayout::new("test_layout");
        assert_eq!(layout.label, "test_layout");
        assert_eq!(layout.entry_count(), 0);

        layout.add_entry(0, "uniform_buffer", ShaderStage::Vertex);
        layout.add_entry(1, "texture", ShaderStage::Fragment);
        assert_eq!(layout.entry_count(), 2);

        let entry = layout.find_entry(0).unwrap();
        assert_eq!(entry.name, "uniform_buffer");
        assert_eq!(entry.visibility, ShaderStage::Vertex);

        assert!(layout.find_entry(99).is_none());
    }

    #[test]
    fn test_pipeline_descriptor() {
        let mut desc = PipelineDescriptor::new("test_pipeline");
        assert_eq!(desc.label, "test_pipeline");
        assert!(!desc.is_complete());

        desc = desc.with_vertex_shader("vertex.wgsl");
        assert!(!desc.is_complete());

        desc.add_bind_group_layout(BindGroupLayout::new("main"));
        assert!(desc.is_complete());

        desc = desc.with_fragment_shader("fragment.wgsl");
        assert_eq!(desc.vertex_shader, Some("vertex.wgsl".to_string()));
        assert_eq!(desc.fragment_shader, Some("fragment.wgsl".to_string()));
    }

    #[test]
    fn test_pipeline_layout_hash() {
        let mut desc = PipelineDescriptor::new("test");
        let mut layout = BindGroupLayout::new("main");
        layout.add_entry(0, "buf", ShaderStage::Vertex);
        desc.add_bind_group_layout(layout);

        desc.compute_layout_hash();
        assert_ne!(desc.pipeline_layout_hash, 0);

        let hash1 = desc.pipeline_layout_hash;
        desc.compute_layout_hash();
        assert_eq!(desc.pipeline_layout_hash, hash1);
    }

    #[test]
    fn test_gpu_surface() {
        let mut surface = GpuSurface::new(1920, 1080);
        assert_eq!(surface.width, 1920);
        assert_eq!(surface.height, 1080);
        assert!((surface.aspect_ratio() - 16.0 / 9.0).abs() < 0.01);
        assert_eq!(surface.pixel_count(), 1920 * 1080);

        surface.resize(0, 0);
        assert_eq!(surface.width, 1);
        assert_eq!(surface.height, 1);

        surface.resize(800, 600);
        assert_eq!(surface.width, 800);
        assert_eq!(surface.height, 600);
    }

    #[test]
    fn test_gpu_pipeline() {
        let mut pipeline = GpuPipeline::new("main", 1024, 768);
        assert!(!pipeline.initialized);

        let mut desc = PipelineDescriptor::new("test");
        desc = desc.with_vertex_shader("vs.wgsl");
        desc.add_bind_group_layout(BindGroupLayout::new("bg"));
        pipeline.descriptor = desc;

        assert!(pipeline.initialize().is_ok());
        assert!(pipeline.initialized);

        assert!(pipeline.begin_render_pass());
        assert!(pipeline.draw(3, 1));
        assert!(pipeline.draw(6, 2));
        assert_eq!(pipeline.end_render_pass(), 2);

        pipeline.resize_surface(640, 480);
        assert_eq!(pipeline.surface.width, 640);
        assert_eq!(pipeline.surface.height, 480);
    }

    #[test]
    fn test_gpu_pipeline_init_failure() {
        let mut pipeline = GpuPipeline::new("incomplete", 100, 100);
        assert!(pipeline.initialize().is_err());
        assert!(!pipeline.initialized);
        assert!(!pipeline.begin_render_pass());
    }

    #[test]
    fn test_gpu_state() {
        let mut state = GpuState::new(GpuBackend::Metal);
        assert_eq!(state.backend, GpuBackend::Metal);
        assert_eq!(state.frame_number, 0);

        let frame = state.begin_frame();
        assert_eq!(frame, 1);

        state.end_frame(1000);
        state.end_frame(2000);

        assert_eq!(state.frame_number, 1);
        assert_eq!(state.total_vertices, 3000);

        state.create_bind_group("main", vec![0, 1, 2]);
        assert_eq!(state.get_bind_group("main"), Some(&vec![0, 1, 2]));
        assert!(state.get_bind_group("missing").is_none());

        assert!(state.remove_bind_group("main"));
        assert!(!state.remove_bind_group("main"));

        state.reset_stats();
        assert_eq!(state.frame_number, 0);
        assert_eq!(state.total_vertices, 0);
    }

    #[test]
    fn test_shader_stage_default() {
        assert_eq!(ShaderStage::default(), ShaderStage::VertexFragment);
    }

    #[test]
    fn test_push_constant_range() {
        let range = PushConstantRange {
            offset: 0,
            size: 128,
            stage: ShaderStage::Vertex,
        };
        assert_eq!(range.offset, 0);
        assert_eq!(range.size, 128);
        assert_eq!(range.stage, ShaderStage::Vertex);
    }
}
