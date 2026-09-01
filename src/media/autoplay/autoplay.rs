#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AutoplaySetting {
    Always,
    Never,
    WithUserGesture,
}

impl Default for AutoplaySetting {
    fn default() -> Self {
        Self::WithUserGesture
    }
}

pub struct AutoplayPolicy;

impl AutoplayPolicy {
    pub fn is_allowed(user_gesture: bool, site_has_played_before: bool, setting: AutoplaySetting) -> bool {
        match setting {
            AutoplaySetting::Always => true,
            AutoplaySetting::Never => false,
            AutoplaySetting::WithUserGesture => user_gesture || site_has_played_before,
        }
    }

    pub fn should_play_on_visibility_change(was_visible: bool, is_visible: bool) -> bool {
        !was_visible && is_visible
    }

    pub fn should_play_on_autoplay_after_load(gesture: bool, setting: AutoplaySetting) -> bool {
        Self::is_allowed(gesture, false, setting)
    }
}
