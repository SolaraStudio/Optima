#[derive(Debug, Clone, Default)]
pub struct Path {
    pub commands: Vec<PathCommand>,
    pub closed: bool,
}

#[derive(Debug, Clone)]
pub enum PathCommand {
    MoveTo {
        x: f32,
        y: f32,
    },
    LineTo {
        x: f32,
        y: f32,
    },
    QuadTo {
        cx: f32,
        cy: f32,
        x: f32,
        y: f32,
    },
    CubicTo {
        cx1: f32,
        cy1: f32,
        cx2: f32,
        cy2: f32,
        x: f32,
        y: f32,
    },
    ArcTo {
        rx: f32,
        ry: f32,
        rotation: f32,
        large_arc: bool,
        sweep: bool,
        x: f32,
        y: f32,
    },
    Close,
}

impl Path {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn move_to(mut self, x: f32, y: f32) -> Self {
        self.commands.push(PathCommand::MoveTo { x, y });
        self
    }
    pub fn line_to(mut self, x: f32, y: f32) -> Self {
        self.commands.push(PathCommand::LineTo { x, y });
        self
    }
    pub fn quad_to(mut self, cx: f32, cy: f32, x: f32, y: f32) -> Self {
        self.commands.push(PathCommand::QuadTo { cx, cy, x, y });
        self
    }
    pub fn cubic_to(mut self, cx1: f32, cy1: f32, cx2: f32, cy2: f32, x: f32, y: f32) -> Self {
        self.commands.push(PathCommand::CubicTo {
            cx1,
            cy1,
            cx2,
            cy2,
            x,
            y,
        });
        self
    }
    pub fn close(mut self) -> Self {
        self.commands.push(PathCommand::Close);
        self.closed = true;
        self
    }
    pub fn rect(x: f32, y: f32, w: f32, h: f32) -> Self {
        Path::new()
            .move_to(x, y)
            .line_to(x + w, y)
            .line_to(x + w, y + h)
            .line_to(x, y + h)
            .close()
    }
    pub fn rounded_rect(x: f32, y: f32, w: f32, h: f32, _rx: f32, _ry: f32) -> Self {
        Path::rect(x, y, w, h)
    }
    pub fn circle(_cx: f32, _cy: f32, _r: f32) -> Self {
        Path::new()
    }
    pub fn ellipse(_cx: f32, _cy: f32, _rx: f32, _ry: f32) -> Self {
        Path::new()
    }
    pub fn is_empty(&self) -> bool {
        self.commands.is_empty()
    }

    pub fn bounds(&self) -> (f32, f32, f32, f32) {
        let mut min_x = f32::MAX;
        let mut min_y = f32::MAX;
        let mut max_x = f32::MIN;
        let mut max_y = f32::MIN;
        for cmd in &self.commands {
            match cmd {
                PathCommand::MoveTo { x, y } | PathCommand::LineTo { x, y } => {
                    min_x = min_x.min(*x);
                    min_y = min_y.min(*y);
                    max_x = max_x.max(*x);
                    max_y = max_y.max(*y);
                }
                _ => {}
            }
        }
        (min_x, min_y, max_x - min_x, max_y - min_y)
    }
}
