#[derive(Debug, Clone, PartialEq)]
#[derive(Default)]
pub enum AutoplayPolicy {
    Allow,
    AllowMuted,
    Deny,
    #[default]
    UserGestureRequired,
}


pub struct AutoplayManager {
    pub policy: AutoplayPolicy,
    pub user_gesture_received: bool,
}

impl AutoplayManager {
    pub fn new(policy: AutoplayPolicy) -> Self {
        AutoplayManager {
            policy,
            user_gesture_received: false,
        }
    }

    pub fn on_user_gesture(&mut self) {
        self.user_gesture_received = true;
    }

    pub fn can_autoplay(&self, muted: bool) -> bool {
        match self.policy {
            AutoplayPolicy::Allow => true,
            AutoplayPolicy::AllowMuted => muted,
            AutoplayPolicy::Deny => false,
            AutoplayPolicy::UserGestureRequired => self.user_gesture_received,
        }
    }
}
