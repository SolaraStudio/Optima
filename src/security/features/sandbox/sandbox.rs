#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FlagKind {
    AllowScripts,
    AllowSameOrigin,
    AllowForms,
    AllowModals,
    AllowDownloads,
    AllowPopups,
}

impl FlagKind {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "allow-scripts" => Some(FlagKind::AllowScripts),
            "allow-same-origin" => Some(FlagKind::AllowSameOrigin),
            "allow-forms" => Some(FlagKind::AllowForms),
            "allow-modals" => Some(FlagKind::AllowModals),
            "allow-downloads" => Some(FlagKind::AllowDownloads),
            "allow-popups" => Some(FlagKind::AllowPopups),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            FlagKind::AllowScripts => "allow-scripts",
            FlagKind::AllowSameOrigin => "allow-same-origin",
            FlagKind::AllowForms => "allow-forms",
            FlagKind::AllowModals => "allow-modals",
            FlagKind::AllowDownloads => "allow-downloads",
            FlagKind::AllowPopups => "allow-popups",
        }
    }
}

pub struct SandboxMode {
    pub flags: Vec<FlagKind>,
    pub enforce_uniqueness: bool,
    pub inherit_origin: bool,
}

impl SandboxMode {
    pub fn new() -> Self {
        SandboxMode {
            flags: Vec::new(),
            enforce_uniqueness: true,
            inherit_origin: false,
        }
    }

    pub fn from_string(input: &str) -> Self {
        let mut mode = SandboxMode::new();
        if input.trim().is_empty() {
            return mode;
        }
        for token in input.split_whitespace() {
            if let Some(flag) = FlagKind::from_str(token) {
                mode.enable(flag);
            }
        }
        mode
    }

    pub fn enable(&mut self, flag: FlagKind) {
        if !self.flags.contains(&flag) {
            self.flags.push(flag);
        }
    }

    pub fn disable(&mut self, flag: FlagKind) {
        self.flags.retain(|f| *f != flag);
    }

    pub fn is_enabled(&self, flag: FlagKind) -> bool {
        self.flags.contains(&flag)
    }

    pub fn allows_scripts(&self) -> bool {
        self.is_enabled(FlagKind::AllowScripts)
    }

    pub fn allows_same_origin(&self) -> bool {
        self.is_enabled(FlagKind::AllowSameOrigin)
    }

    pub fn allows_forms(&self) -> bool {
        self.is_enabled(FlagKind::AllowForms)
    }

    pub fn allows_modals(&self) -> bool {
        self.is_enabled(FlagKind::AllowModals)
    }

    pub fn is_fully_restricted(&self) -> bool {
        self.flags.is_empty()
    }

    pub fn has_conflicting_flags(&self) -> bool {
        self.allows_scripts() && self.allows_same_origin()
    }

    pub fn to_string(&self) -> String {
        self.flags
            .iter()
            .map(|f| f.as_str().to_string())
            .collect::<Vec<_>>()
            .join(" ")
    }

    pub fn len(&self) -> usize {
        self.flags.len()
    }

    pub fn is_empty(&self) -> bool {
        self.flags.is_empty()
    }
}

impl Default for SandboxMode {
    fn default() -> Self {
        SandboxMode::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_flags() {
        let mode = SandboxMode::from_string("allow-scripts allow-forms");
        assert!(mode.allows_scripts());
        assert!(mode.allows_forms());
        assert!(!mode.allows_same_origin());
    }

    #[test]
    fn empty_is_fully_restricted() {
        let mode = SandboxMode::from_string("");
        assert!(mode.is_fully_restricted());
        assert!(mode.is_empty());
    }

    #[test]
    fn enables_and_disables() {
        let mut mode = SandboxMode::new();
        mode.enable(FlagKind::AllowModals);
        assert!(mode.allows_modals());
        mode.disable(FlagKind::AllowModals);
        assert!(!mode.allows_modals());
    }

    #[test]
    fn enable_is_idempotent() {
        let mut mode = SandboxMode::new();
        mode.enable(FlagKind::AllowScripts);
        mode.enable(FlagKind::AllowScripts);
        assert_eq!(mode.len(), 1);
    }

    #[test]
    fn conflicts_detected() {
        let mode = SandboxMode::from_string("allow-scripts allow-same-origin");
        assert!(mode.has_conflicting_flags());
        let clean = SandboxMode::from_string("allow-scripts only");
        assert!(!clean.has_conflicting_flags());
    }

    #[test]
    fn string_roundtrip() {
        let mode = SandboxMode::from_string("allow-modals allow-popups");
        assert_eq!(mode.to_string(), "allow-modals allow-popups");
    }

    #[test]
    fn ignores_unknown_tokens() {
        let mode = SandboxMode::from_string("allow-scripts some-random-token");
        assert!(mode.allows_scripts());
        assert_eq!(mode.len(), 1);
    }
}
