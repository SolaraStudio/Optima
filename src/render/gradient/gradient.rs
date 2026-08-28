use vello::peniko::Color;

pub struct GradientStop {
    pub position: f32,
    pub color: Color,
}

pub struct LinearGradient {
    pub start_x: f32,
    pub start_y: f32,
    pub end_x: f32,
    pub end_y: f32,
    pub stops: Vec<GradientStop>,
}

impl LinearGradient {
    pub fn new(start_x: f32, start_y: f32, end_x: f32, end_y: f32) -> Self {
        Self {
            start_x,
            start_y,
            end_x,
            end_y,
            stops: Vec::new(),
        }
    }

    pub fn add_stop(&mut self, position: f32, color: Color) {
        self.stops.push(GradientStop { position, color });
    }

    pub fn get_color_at(&self, t: f32) -> Color {
        if self.stops.is_empty() {
            return Color::BLACK;
        }
        if t <= self.stops[0].position {
            return self.stops[0].color;
        }
        for i in 0..self.stops.len() - 1 {
            let a = &self.stops[i];
            let b = &self.stops[i + 1];
            if t >= a.position && t <= b.position {
                let p = (t - a.position) / (b.position - a.position);
                let r = a.color.r + (b.color.r - a.color.r) * p;
                let g = a.color.g + (b.color.g - a.color.g) * p;
                let b_val = a.color.b + (b.color.b - a.color.b) * p;
                let a_val = a.color.a + (b.color.a - a.color.a) * p;
                return Color::new(r, g, b_val, a_val);
            }
        }
        self.stops.last().unwrap().color
    }
}
