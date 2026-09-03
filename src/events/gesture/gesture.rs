#[derive(Debug, Clone)]
pub enum GestureEvent {
    Tap(GestureData),
    DoubleTap(GestureData),
    LongPress(GestureData),
    Swipe(GestureData, SwipeDirection),
    Pinch(GestureData, f32),
    Rotate(GestureData, f32),
    Pan(GestureData, f32, f32),
}

#[derive(Debug, Clone)]
pub struct GestureData {
    pub x: f32,
    pub y: f32,
    pub client_x: f32,
    pub client_y: f32,
    pub screen_x: f32,
    pub screen_y: f32,
    pub target_id: Option<String>,
    pub pointer_count: u32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SwipeDirection {
    Left,
    Right,
    Up,
    Down,
    LeftUp,
    RightUp,
    LeftDown,
    RightDown,
}

impl GestureData {
    pub fn new(x: f32, y: f32) -> Self {
        GestureData {
            x,
            y,
            client_x: x,
            client_y: y,
            screen_x: x,
            screen_y: y,
            target_id: None,
            pointer_count: 1,
        }
    }

    pub fn with_target(mut self, target: &str) -> Self {
        self.target_id = Some(target.to_string());
        self
    }

    pub fn with_pointer_count(mut self, count: u32) -> Self {
        self.pointer_count = count;
        self
    }
}

impl SwipeDirection {
    pub fn from_delta(dx: f32, dy: f32) -> Self {
        let abs_dx = dx.abs();
        let abs_dy = dy.abs();
        if abs_dx > abs_dy {
            if dx > 0.0 {
                SwipeDirection::Right
            } else {
                SwipeDirection::Left
            }
        } else {
            if dy > 0.0 {
                SwipeDirection::Down
            } else {
                SwipeDirection::Up
            }
        }
    }
}
