pub struct AndroidLifecycle {
    pub is_active: bool,
    pub is_visible: bool,
}

impl AndroidLifecycle {
    pub fn new() -> Self {
        Self {
            is_active: true,
            is_visible: true,
        }
    }

    pub fn on_resume(&mut self) {
        self.is_active = true;
        self.is_visible = true;
    }

    pub fn on_pause(&mut self) {
        self.is_active = false;
        self.is_visible = false;
    }

    pub fn on_stop(&mut self) {
        self.is_visible = false;
    }

    pub fn on_start(&mut self) {
        self.is_visible = true;
    }

    pub fn is_active(&self) -> bool {
        self.is_active
    }

    pub fn is_visible(&self) -> bool {
        self.is_visible
    }
}
