#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
    Back,
    Forward,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MouseAction {
    Down,
    Up,
    Move,
    Click,
    DoubleClick,
    Drag,
    DragStart,
    DragEnd,
    Enter,
    Leave,
    Over,
    Out,
    Wheel,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct ModifierKeys {
    pub shift: bool,
    pub ctrl: bool,
    pub alt: bool,
    pub meta: bool,
}

impl ModifierKeys {
    pub fn is_pressed(&self) -> bool {
        self.shift || self.ctrl || self.alt || self.meta
    }

    pub fn any(&self) -> bool {
        self.is_pressed()
    }

    pub fn none(&self) -> bool {
        !self.is_pressed()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct MouseEvent {
    pub x: f32,
    pub y: f32,
    pub client_x: f32,
    pub client_y: f32,
    pub screen_x: f32,
    pub screen_y: f32,
    pub button: MouseButton,
    pub action: MouseAction,
    pub modifiers: ModifierKeys,
    pub click_count: u32,
    pub delta_x: f32,
    pub delta_y: f32,
}

impl MouseEvent {
    pub fn new(x: f32, y: f32, button: MouseButton, action: MouseAction) -> Self {
        Self {
            x,
            y,
            client_x: x,
            client_y: y,
            screen_x: x,
            screen_y: y,
            button,
            action,
            modifiers: ModifierKeys::default(),
            click_count: 1,
            delta_x: 0.0,
            delta_y: 0.0,
        }
    }

    pub fn with_modifiers(mut self, modifiers: ModifierKeys) -> Self {
        self.modifiers = modifiers;
        self
    }

    pub fn with_click_count(mut self, count: u32) -> Self {
        self.click_count = count;
        self
    }

    pub fn with_delta(mut self, dx: f32, dy: f32) -> Self {
        self.delta_x = dx;
        self.delta_y = dy;
        self
    }

    pub fn with_client(mut self, client_x: f32, client_y: f32) -> Self {
        self.client_x = client_x;
        self.client_y = client_y;
        self
    }

    pub fn with_screen(mut self, screen_x: f32, screen_y: f32) -> Self {
        self.screen_x = screen_x;
        self.screen_y = screen_y;
        self
    }

    pub fn is_left(&self) -> bool {
        matches!(self.button, MouseButton::Left)
    }

    pub fn is_right(&self) -> bool {
        matches!(self.button, MouseButton::Right)
    }

    pub fn is_middle(&self) -> bool {
        matches!(self.button, MouseButton::Middle)
    }

    pub fn is_down(&self) -> bool {
        matches!(self.action, MouseAction::Down)
    }

    pub fn is_up(&self) -> bool {
        matches!(self.action, MouseAction::Up)
    }

    pub fn is_move(&self) -> bool {
        matches!(self.action, MouseAction::Move)
    }

    pub fn is_click(&self) -> bool {
        matches!(self.action, MouseAction::Click)
    }

    pub fn is_double_click(&self) -> bool {
        matches!(self.action, MouseAction::DoubleClick)
    }

    pub fn is_drag(&self) -> bool {
        matches!(self.action, MouseAction::Drag)
    }

    pub fn is_wheel(&self) -> bool {
        matches!(self.action, MouseAction::Wheel)
    }
}
