pub struct DevToolsFrontend {
    pub visible: bool,
    pub selected_panel: Panel,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Panel {
    Elements,
    Console,
    Network,
    Performance,
    Memory,
    Sources,
    Application,
    Storage,
}

impl Default for DevToolsFrontend {
    fn default() -> Self {
        DevToolsFrontend { visible: false, selected_panel: Panel::Elements }
    }
}

impl DevToolsFrontend {
    pub fn new() -> Self { Self::default() }
    pub fn show(&mut self) { self.visible = true; }
    pub fn hide(&mut self) { self.visible = false; }
    pub fn toggle(&mut self) { self.visible = !self.visible; }
    pub fn select_panel(&mut self, panel: Panel) { self.selected_panel = panel; }
}
