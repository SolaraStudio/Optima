use crate::render::paint::PaintCommand;
use crate::render::path::Path;
use crate::render::transform::RenderTransform;

pub struct VelloRenderer {
    pub width: u32,
    pub height: u32,
    pub transform_stack: Vec<RenderTransform>,
}

impl VelloRenderer {
    pub fn new(width: u32, height: u32) -> Self {
        VelloRenderer {
            width,
            height,
            transform_stack: vec![RenderTransform::identity()],
        }
    }

    pub fn push_transform(&mut self, transform: RenderTransform) {
        let parent = self.transform_stack.last().cloned().unwrap_or_default();
        self.transform_stack.push(parent.multiply(&transform));
    }

    pub fn pop_transform(&mut self) {
        self.transform_stack.pop();
    }

    pub fn current_transform(&self) -> &RenderTransform {
        self.transform_stack.last().unwrap_or(&RenderTransform {
            matrix: [1.0, 0.0, 0.0, 1.0, 0.0, 0.0],
        })
    }

    pub fn render_rect(&self, x: f32, y: f32, w: f32, h: f32, paint: &PaintCommand) -> RenderOp {
        RenderOp::Rect {
            x,
            y,
            w,
            h,
            paint: (*paint).clone(),
            transform: self.current_transform().clone(),
        }
    }

    pub fn render_path(&self, path: &Path, paint: &PaintCommand) -> RenderOp {
        RenderOp::Path {
            path: (*path).clone(),
            paint: (*paint).clone(),
            transform: self.current_transform().clone(),
        }
    }

    pub fn render_text(&self, x: f32, y: f32, text: &str, font_size: f32) -> RenderOp {
        RenderOp::Text {
            x,
            y,
            text: text.to_string(),
            font_size,
            transform: self.current_transform().clone(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum RenderOp {
    Rect {
        x: f32,
        y: f32,
        w: f32,
        h: f32,
        paint: PaintCommand,
        transform: RenderTransform,
    },
    Path {
        path: Path,
        paint: PaintCommand,
        transform: RenderTransform,
    },
    Text {
        x: f32,
        y: f32,
        text: String,
        font_size: f32,
        transform: RenderTransform,
    },
    Group {
        ops: Vec<RenderOp>,
    },
}
