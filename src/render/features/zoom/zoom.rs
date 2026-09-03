#[derive(Debug, Clone)]
pub struct ViewportZoom {
    pub factor: f32,
    pub min_factor: f32,
    pub max_factor: f32,
    pub center_x: f64,
    pub center_y: f64,
    pub smoothing: f32,
    pub target_factor: f32,
    pub pivot_x: f64,
    pub pivot_y: f64,
}

impl Default for ViewportZoom {
    fn default() -> Self {
        ViewportZoom {
            factor: 1.0,
            min_factor: 0.1,
            max_factor: 10.0,
            center_x: 0.0,
            center_y: 0.0,
            smoothing: 0.15,
            target_factor: 1.0,
            pivot_x: 0.0,
            pivot_y: 0.0,
        }
    }
}

impl ViewportZoom {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_range(min: f32, max: f32) -> Self {
        ViewportZoom {
            min_factor: min.max(0.01),
            max_factor: max.max(min.max(0.01)),
            factor: 1.0f32.clamp(min.max(0.01), max.max(min.max(0.01))),
            target_factor: 1.0f32.clamp(min.max(0.01), max.max(min.max(0.01))),
            ..Default::default()
        }
    }

    pub fn with_smoothing(mut self, smoothing: f32) -> Self {
        self.smoothing = smoothing.clamp(0.01, 1.0);
        self
    }

    pub fn clamp_factor(&self, factor: f32) -> f32 {
        factor.clamp(self.min_factor, self.max_factor)
    }

    pub fn set_factor(&mut self, factor: f32) {
        self.factor = self.clamp_factor(factor);
        self.target_factor = self.factor;
    }

    pub fn zoom_to(&mut self, factor: f32) {
        self.target_factor = self.clamp_factor(factor);
    }

    pub fn zoom_in(&mut self, delta: f32) {
        let new_target = self.target_factor * (1.0 + delta);
        self.target_factor = self.clamp_factor(new_target);
    }

    pub fn zoom_out(&mut self, delta: f32) {
        let new_target = self.target_factor / (1.0 + delta);
        self.target_factor = self.clamp_factor(new_target);
    }

    pub fn zoom_at(&mut self, delta: f32, px: f64, py: f64) {
        self.pivot_x = px;
        self.pivot_y = py;
        let new_target = self.target_factor * (1.0 + delta);
        self.target_factor = self.clamp_factor(new_target);
    }

    pub fn reset(&mut self) {
        self.factor = 1.0;
        self.target_factor = 1.0;
        self.center_x = 0.0;
        self.center_y = 0.0;
        self.pivot_x = 0.0;
        self.pivot_y = 0.0;
    }

    pub fn set_center(&mut self, x: f64, y: f64) {
        self.center_x = x;
        self.center_y = y;
    }

    pub fn update(&mut self, dt: f32) {
        let diff = self.target_factor - self.factor;
        if diff.abs() > 0.0001 {
            self.factor += diff * self.smoothing * dt * 60.0;
            if (self.target_factor - self.factor).abs() < 0.0001 {
                self.factor = self.target_factor;
            }
        }
    }

    pub fn is_animating(&self) -> bool {
        (self.target_factor - self.factor).abs() > 0.0001
    }

    pub fn screen_to_world(&self, sx: f64, sy: f64) -> (f64, f64) {
        let wx = (sx - self.center_x) / self.factor as f64 + self.pivot_x;
        let wy = (sy - self.center_y) / self.factor as f64 + self.pivot_y;
        (wx, wy)
    }

    pub fn world_to_screen(&self, wx: f64, wy: f64) -> (f64, f64) {
        let sx = (wx - self.pivot_x) * self.factor as f64 + self.center_x;
        let sy = (wy - self.pivot_y) * self.factor as f64 + self.center_y;
        (sx, sy)
    }

    pub fn visible_width(&self, viewport_width: u32) -> f64 {
        viewport_width as f64 / self.factor as f64
    }

    pub fn visible_height(&self, viewport_height: u32) -> f64 {
        viewport_height as f64 / self.factor as f64
    }

    pub fn percentage(&self) -> f32 {
        self.factor * 100.0
    }

    pub fn scale(&self) -> f32 {
        self.factor
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zoom_default() {
        let z = ViewportZoom::default();
        assert_eq!(z.factor, 1.0);
        assert_eq!(z.min_factor, 0.1);
        assert_eq!(z.max_factor, 10.0);
        assert!(!z.is_animating());
    }

    #[test]
    fn test_zoom_with_range() {
        let z = ViewportZoom::with_range(0.5, 5.0);
        assert_eq!(z.min_factor, 0.5);
        assert_eq!(z.max_factor, 5.0);
    }

    #[test]
    fn test_zoom_set_factor() {
        let mut z = ViewportZoom::new();
        z.set_factor(3.0);
        assert_eq!(z.factor, 3.0);
        assert_eq!(z.target_factor, 3.0);

        z.set_factor(100.0);
        assert_eq!(z.factor, z.max_factor);
    }

    #[test]
    fn test_zoom_in_out() {
        let mut z = ViewportZoom::new();
        z.zoom_in(0.1);
        assert!((z.target_factor - 1.1).abs() < 0.001);

        z.zoom_out(0.5);
        assert!((z.target_factor - 1.1 / 1.5).abs() < 0.001);
    }

    #[test]
    fn test_zoom_at() {
        let mut z = ViewportZoom::new();
        z.zoom_at(0.2, 100.0, 200.0);
        assert_eq!(z.pivot_x, 100.0);
        assert_eq!(z.pivot_y, 200.0);
        assert!((z.target_factor - 1.2).abs() < 0.001);
    }

    #[test]
    fn test_zoom_reset() {
        let mut z = ViewportZoom::new();
        z.zoom_in(0.5);
        z.set_center(10.0, 20.0);
        z.reset();
        assert_eq!(z.factor, 1.0);
        assert_eq!(z.target_factor, 1.0);
        assert_eq!(z.center_x, 0.0);
    }

    #[test]
    fn test_zoom_update() {
        let mut z = ViewportZoom::new();
        z.set_factor(1.0);
        z.zoom_to(2.0);
        assert!(z.is_animating());

        z.update(1.0 / 60.0);
        assert!((z.target_factor - z.factor).abs() > 0.0);

        for _ in 0..300 {
            z.update(1.0 / 60.0);
        }
        assert!(!z.is_animating());
        assert!((z.factor - 2.0).abs() < 0.01);
    }

    #[test]
    fn test_zoom_clamp() {
        let mut z = ViewportZoom::with_range(0.25, 4.0);
        z.set_factor(0.01);
        assert_eq!(z.factor, 0.25);

        z.set_factor(100.0);
        assert_eq!(z.factor, 4.0);
    }

    #[test]
    fn test_screen_world_conversion() {
        let mut z = ViewportZoom::new();
        z.set_factor(2.0);
        z.set_center(0.0, 0.0);

        let (wx, wy) = z.screen_to_world(100.0, 100.0);
        assert!((wx - 50.0).abs() < 0.01);
        assert!((wy - 50.0).abs() < 0.01);

        let (sx, sy) = z.world_to_screen(50.0, 50.0);
        assert!((sx - 100.0).abs() < 0.01);
        assert!((sy - 100.0).abs() < 0.01);
    }

    #[test]
    fn test_visible_dimensions() {
        let z = ViewportZoom::new();
        let _ = z.clone();

        let mut z2 = ViewportZoom::new();
        z2.set_factor(2.0);
        assert!((z2.visible_width(1000) - 500.0).abs() < 0.01);
        assert!((z2.visible_height(800) - 400.0).abs() < 0.01);
    }

    #[test]
    fn test_percentage() {
        let mut z = ViewportZoom::new();
        z.set_factor(1.5);
        assert!((z.percentage() - 150.0).abs() < 0.01);
    }
}
