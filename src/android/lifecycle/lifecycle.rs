#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LifecycleState {
    Created,
    Started,
    Resumed,
    Paused,
    Stopped,
    Destroyed,
}

pub struct AndroidLifecycle {
    pub state: LifecycleState,
    pub is_active: bool,
    pub is_visible: bool,
}

impl Default for AndroidLifecycle {
    fn default() -> Self {
        Self::new()
    }
}

impl AndroidLifecycle {
    pub fn new() -> Self {
        AndroidLifecycle {
            state: LifecycleState::Created,
            is_active: true,
            is_visible: true,
        }
    }

    pub fn on_create(&mut self) {
        self.state = LifecycleState::Created;
    }

    pub fn on_start(&mut self) {
        self.state = LifecycleState::Started;
        self.is_visible = true;
    }

    pub fn on_resume(&mut self) {
        self.state = LifecycleState::Resumed;
        self.is_active = true;
        self.is_visible = true;
    }

    pub fn on_pause(&mut self) {
        self.state = LifecycleState::Paused;
        self.is_active = false;
    }

    pub fn on_stop(&mut self) {
        self.state = LifecycleState::Stopped;
        self.is_visible = false;
    }

    pub fn on_destroy(&mut self) {
        self.state = LifecycleState::Destroyed;
        self.is_active = false;
        self.is_visible = false;
    }

    pub fn is_active(&self) -> bool {
        self.is_active
    }

    pub fn is_visible(&self) -> bool {
        self.is_visible
    }

    pub fn is_resumed(&self) -> bool {
        matches!(self.state, LifecycleState::Resumed)
    }

    pub fn can_render(&self) -> bool {
        self.is_active && self.is_visible
    }
}
