#[derive(Debug, Clone)]
pub enum FocusEvent {
    Focus(FocusData),
    Blur(FocusData),
    FocusIn(FocusData),
    FocusOut(FocusData),
}

#[derive(Debug, Clone)]
pub struct FocusData {
    pub target_id: Option<String>,
    pub related_target_id: Option<String>,
    pub source_capabilities: Option<String>,
}

impl Default for FocusData {
    fn default() -> Self {
        Self::new()
    }
}

impl FocusData {
    pub fn new() -> Self {
        FocusData {
            target_id: None,
            related_target_id: None,
            source_capabilities: None,
        }
    }

    pub fn with_target(mut self, target: &str) -> Self {
        self.target_id = Some(target.to_string());
        self
    }

    pub fn with_related_target(mut self, related: &str) -> Self {
        self.related_target_id = Some(related.to_string());
        self
    }
}
