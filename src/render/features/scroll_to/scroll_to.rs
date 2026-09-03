#[derive(Debug, Clone, PartialEq)]
pub enum ScrollTarget {
    ElementId(String),
    ClassName(String),
    TagName(String),
    Position(f32),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScrollAlignment {
    Top,
    Center,
    Bottom,
    Nearest,
}

impl Default for ScrollAlignment {
    fn default() -> Self {
        ScrollAlignment::Top
    }
}

#[derive(Debug, Clone)]
pub struct ScrollToConfig {
    pub target: ScrollTarget,
    pub offset_x: f32,
    pub offset_y: f32,
    pub smooth: bool,
    pub duration_ms: f32,
    pub alignment: ScrollAlignment,
}

impl Default for ScrollToConfig {
    fn default() -> Self {
        ScrollToConfig {
            target: ScrollTarget::Position(0.0),
            offset_x: 0.0,
            offset_y: 0.0,
            smooth: true,
            duration_ms: 300.0,
            alignment: ScrollAlignment::Top,
        }
    }
}

impl ScrollToConfig {
    pub fn new(target: ScrollTarget) -> Self {
        ScrollToConfig {
            target,
            ..Default::default()
        }
    }

    pub fn with_offset(mut self, x: f32, y: f32) -> Self {
        self.offset_x = x;
        self.offset_y = y;
        self
    }

    pub fn with_smooth(mut self, smooth: bool, duration_ms: f32) -> Self {
        self.smooth = smooth;
        self.duration_ms = duration_ms.max(1.0);
        self
    }

    pub fn with_alignment(mut self, alignment: ScrollAlignment) -> Self {
        self.alignment = alignment;
        self
    }
}

#[derive(Debug, Clone)]
pub struct ScrollTargetInfo {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub found: bool,
}

impl Default for ScrollTargetInfo {
    fn default() -> Self {
        ScrollTargetInfo {
            x: 0.0,
            y: 0.0,
            width: 0.0,
            height: 0.0,
            found: false,
        }
    }
}

impl ScrollTargetInfo {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        ScrollTargetInfo {
            x,
            y,
            width,
            height,
            found: true,
        }
    }

    pub fn center_y(&self) -> f32 {
        self.y + self.height / 2.0
    }

    pub fn center_x(&self) -> f32 {
        self.x + self.width / 2.0
    }

    pub fn bottom(&self) -> f32 {
        self.y + self.height
    }

    pub fn right(&self) -> f32 {
        self.x + self.width
    }
}

#[derive(Debug, Clone)]
pub struct ScrollToState {
    pub current_x: f32,
    pub current_y: f32,
    pub target_x: f32,
    pub target_y: f32,
    pub viewport_width: f32,
    pub viewport_height: f32,
    pub content_width: f32,
    pub content_height: f32,
    pub pending_config: Option<ScrollToConfig>,
    pub is_scrolling: bool,
    pub scroll_history: Vec<(f32, f32)>,
    pub max_history: usize,
}

impl Default for ScrollToState {
    fn default() -> Self {
        ScrollToState {
            current_x: 0.0,
            current_y: 0.0,
            target_x: 0.0,
            target_y: 0.0,
            viewport_width: 800.0,
            viewport_height: 600.0,
            content_width: 0.0,
            content_height: 0.0,
            pending_config: None,
            is_scrolling: false,
            scroll_history: Vec::new(),
            max_history: 100,
        }
    }
}

impl ScrollToState {
    pub fn new(viewport_width: f32, viewport_height: f32) -> Self {
        ScrollToState {
            viewport_width,
            viewport_height,
            ..Default::default()
        }
    }

    pub fn set_content_size(&mut self, width: f32, height: f32) {
        self.content_width = width;
        self.content_height = height;
    }

    pub fn resolve_alignment_offset(&self, alignment: ScrollAlignment, element_height: f32) -> f32 {
        match alignment {
            ScrollAlignment::Top => 0.0,
            ScrollAlignment::Center => (self.viewport_height - element_height) / 2.0,
            ScrollAlignment::Bottom => self.viewport_height - element_height,
            ScrollAlignment::Nearest => {
                if element_height > self.viewport_height {
                    0.0
                } else {
                    (self.viewport_height - element_height) / 2.0
                }
            }
        }
    }

    pub fn calculate_scroll_position(
        &self,
        target: &ScrollTargetInfo,
        config: &ScrollToConfig,
    ) -> (f32, f32) {
        if !target.found {
            return (self.current_x, self.current_y);
        }
        let align_offset = self.resolve_alignment_offset(config.alignment, target.height);
        let mut sx = target.x + config.offset_x;
        let mut sy = target.y + config.offset_y - align_offset;

        sx = sx.max(0.0).min((self.content_width - self.viewport_width).max(0.0));
        sy = sy.max(0.0).min((self.content_height - self.viewport_height).max(0.0));

        (sx, sy)
    }

    pub fn scroll_to_position(&mut self, x: f32, y: f32) {
        self.record_history();
        self.target_x = x.max(0.0).min((self.content_width - self.viewport_width).max(0.0));
        self.target_y = y.max(0.0).min((self.content_height - self.viewport_height).max(0.0));
        self.is_scrolling = true;
    }

    pub fn scroll_to_element(&mut self, info: &ScrollTargetInfo, config: ScrollToConfig) {
        let (sx, sy) = self.calculate_scroll_position(info, &config);
        self.pending_config = Some(config);
        self.scroll_to_position(sx, sy);
    }

    pub fn scroll_into_view(&mut self, info: &ScrollTargetInfo) {
        if !info.found {
            return;
        }

        let mut needs_scroll = false;
        let mut sx = self.current_x;
        let mut sy = self.current_y;

        if info.y < self.current_y {
            sy = info.y;
            needs_scroll = true;
        } else if info.bottom() > self.current_y + self.viewport_height {
            sy = info.bottom() - self.viewport_height;
            needs_scroll = true;
        }

        if info.x < self.current_x {
            sx = info.x;
            needs_scroll = true;
        } else if info.right() > self.current_x + self.viewport_width {
            sx = info.right() - self.viewport_width;
            needs_scroll = true;
        }

        if needs_scroll {
            self.scroll_to_position(sx, sy);
        }
    }

    pub fn update(&mut self, dt_ms: f32) {
        if !self.is_scrolling {
            return;
        }

        let speed = 0.15;
        let dx = self.target_x - self.current_x;
        let dy = self.target_y - self.current_y;

        self.current_x += dx * speed * dt_ms / 16.0;
        self.current_y += dy * speed * dt_ms / 16.0;

        if dx.abs() < 0.5 && dy.abs() < 0.5 {
            self.current_x = self.target_x;
            self.current_y = self.target_y;
            self.is_scrolling = false;
            self.pending_config = None;
        }
    }

    pub fn record_history(&mut self) {
        self.scroll_history.push((self.current_x, self.current_y));
        if self.scroll_history.len() > self.max_history {
            self.scroll_history.remove(0);
        }
    }

    pub fn can_go_back(&self) -> bool {
        !self.scroll_history.is_empty()
    }

    pub fn go_back(&mut self) -> bool {
        if let Some((x, y)) = self.scroll_history.pop() {
            self.target_x = x;
            self.target_y = y;
            self.is_scrolling = true;
            true
        } else {
            false
        }
    }

    pub fn progress(&self) -> f32 {
        let dx = self.target_x - self.current_x;
        let dy = self.target_y - self.current_y;
        let dist = (dx * dx + dy * dy).sqrt();
        if dist < 0.5 {
            1.0
        } else {
            let total = ((self.target_x).powi(2) + (self.target_y).powi(2)).sqrt();
            if total > 0.0 {
                (1.0 - dist / total).clamp(0.0, 1.0)
            } else {
                1.0
            }
        }
    }

    pub fn clamp_to_bounds(&mut self) {
        self.current_x = self.current_x.max(0.0).min((self.content_width - self.viewport_width).max(0.0));
        self.current_y = self.current_y.max(0.0).min((self.content_height - self.viewport_height).max(0.0));
        self.target_x = self.target_x.max(0.0).min((self.content_width - self.viewport_width).max(0.0));
        self.target_y = self.target_y.max(0.0).min((self.content_height - self.viewport_height).max(0.0));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scroll_target() {
        let t = ScrollTargetInfo::new(100.0, 200.0, 50.0, 30.0);
        assert!(t.found);
        assert_eq!(t.center_x(), 125.0);
        assert_eq!(t.center_y(), 215.0);
        assert_eq!(t.bottom(), 230.0);
        assert_eq!(t.right(), 150.0);
    }

    #[test]
    fn test_scroll_target_not_found() {
        let t = ScrollTargetInfo::default();
        assert!(!t.found);
    }

    #[test]
    fn test_scroll_to_config() {
        let config = ScrollToConfig::new(ScrollTarget::ElementId("main".to_string()))
            .with_offset(0.0, -50.0)
            .with_smooth(true, 500.0)
            .with_alignment(ScrollAlignment::Center);

        assert_eq!(config.offset_y, -50.0);
        assert!(config.smooth);
        assert_eq!(config.duration_ms, 500.0);
        assert_eq!(config.alignment, ScrollAlignment::Center);
    }

    #[test]
    fn test_scroll_to_state() {
        let mut state = ScrollToState::new(800.0, 600.0);
        state.set_content_size(2000.0, 3000.0);

        assert_eq!(state.viewport_width, 800.0);
        assert_eq!(state.content_height, 3000.0);
    }

    #[test]
    fn test_scroll_to_position() {
        let mut state = ScrollToState::new(800.0, 600.0);
        state.set_content_size(2000.0, 3000.0);
        state.scroll_to_position(100.0, 200.0);

        assert!(state.is_scrolling);
        assert_eq!(state.target_x, 100.0);
        assert_eq!(state.target_y, 200.0);
    }

    #[test]
    fn test_scroll_to_position_bounds() {
        let mut state = ScrollToState::new(800.0, 600.0);
        state.set_content_size(1000.0, 1000.0);

        state.scroll_to_position(-100.0, 2000.0);
        assert_eq!(state.target_x, 0.0);
        assert_eq!(state.target_y, 400.0);

        state.scroll_to_position(500.0, -50.0);
        assert_eq!(state.target_x, 200.0);
        assert_eq!(state.target_y, 0.0);
    }

    #[test]
    fn test_scroll_update() {
        let mut state = ScrollToState::new(800.0, 600.0);
        state.set_content_size(2000.0, 2000.0);
        state.scroll_to_position(1000.0, 1000.0);
        assert!(state.is_scrolling);

        for _ in 0..200 {
            state.update(16.0);
        }
        assert!(!state.is_scrolling);
        assert!((state.current_x - 1000.0).abs() < 1.0);
    }

    #[test]
    fn test_scroll_into_view() {
        let mut state = ScrollToState::new(800.0, 600.0);
        state.set_content_size(2000.0, 3000.0);

        let above = ScrollTargetInfo::new(100.0, -50.0, 50.0, 30.0);
        state.scroll_into_view(&above);
        assert!(state.is_scrolling);

        state.current_x = 0.0;
        state.current_y = 500.0;
        let below = ScrollTargetInfo::new(100.0, 1200.0, 50.0, 30.0);
        state.scroll_into_view(&below);
        assert!(state.is_scrolling);
    }

    #[test]
    fn test_alignment_offsets() {
        let state = ScrollToState::new(800.0, 600.0);
        assert_eq!(state.resolve_alignment_offset(ScrollAlignment::Top, 100.0), 0.0);
        assert_eq!(state.resolve_alignment_offset(ScrollAlignment::Center, 100.0), 250.0);
        assert_eq!(state.resolve_alignment_offset(ScrollAlignment::Bottom, 100.0), 500.0);
    }

    #[test]
    fn test_scroll_history() {
        let mut state = ScrollToState::new(800.0, 600.0);
        state.set_content_size(2000.0, 2000.0);

        assert!(!state.can_go_back());

        state.record_history();
        state.current_x = 100.0;
        state.record_history();
        state.current_x = 200.0;
        state.record_history();

        assert!(state.can_go_back());
        assert!(state.go_back());
        assert_eq!(state.current_x, 100.0);
    }

    #[test]
    fn test_clamp_to_bounds() {
        let mut state = ScrollToState::new(800.0, 600.0);
        state.set_content_size(1000.0, 1000.0);
        state.current_x = -50.0;
        state.current_y = 500.0;
        state.clamp_to_bounds();
        assert_eq!(state.current_x, 0.0);
        assert_eq!(state.current_y, 400.0);
    }

    #[test]
    fn test_scroll_target_enums() {
        let id = ScrollTarget::ElementId("btn".to_string());
        let cls = ScrollTarget::ClassName("container".to_string());
        let tag = ScrollTarget::TagName("div".to_string());
        let pos = ScrollTarget::Position(42.0);

        assert!(matches!(id, ScrollTarget::ElementId(_)));
        assert!(matches!(cls, ScrollTarget::ClassName(_)));
        assert!(matches!(tag, ScrollTarget::TagName(_)));
        assert!(matches!(pos, ScrollTarget::Position(42.0)));
    }
}
