use crate::css::colors::Color;

#[derive(Debug, Clone)]
pub struct GradientStop {
    pub position: f32,
    pub color: Color,
}

#[derive(Debug, Clone)]
pub enum GradientKind {
    Linear { angle: f32 },
    Radial { cx: f32, cy: f32, r: f32 },
}

#[derive(Debug, Clone)]
pub struct Gradient {
    pub kind: GradientKind,
    pub stops: Vec<GradientStop>,
}

impl Gradient {
    pub fn linear(angle: f32) -> Self { Gradient { kind: GradientKind::Linear { angle }, stops: Vec::new() } }
    pub fn radial() -> Self { Gradient { kind: GradientKind::Radial { cx: 0.5, cy: 0.5, r: 0.5 }, stops: Vec::new() } }

    pub fn add_stop(mut self, position: f32, color: Color) -> Self {
        self.stops.push(GradientStop { position, color }); self
    }

    pub fn color_at(&self, t: f32) -> Color {
        if self.stops.is_empty() { return Color::default(); }
        if self.stops.len() == 1 { return self.stops[0].color; }
        let t = t.clamp(0.0, 1.0);
        for i in 0..self.stops.len()-1 {
            if t >= self.stops[i].position && t <= self.stops[i+1].position {
                let range = self.stops[i+1].position - self.stops[i].position;
                let local_t = if range > 0.0 { (t - self.stops[i].position) / range } else { 0.0 };
                return lerp_color(self.stops[i].color, self.stops[i+1].color, local_t);
            }
        }
        self.stops.last().unwrap().color
    }
}

fn lerp_color(a: Color, b: Color, t: f32) -> Color {
    Color::new(
        a.r + (b.r - a.r) * t,
        a.g + (b.g - a.g) * t,
        a.b + (b.b - a.b) * t,
        a.a + (b.a - a.a) * t,
    )
}
