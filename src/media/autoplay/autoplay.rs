pub struct AutoplayPolicy;

impl AutoplayPolicy {
    pub fn is_allowed(user_gesture: bool, site_has_media: bool, site_has_played_before: bool) -> bool {
        if user_gesture {
            return true;
        }
        if site_has_played_before {
            return true;
        }
        if site_has_media {
            return false;
        }
        false
    }

    pub fn should_play_on_visibility_change(was_visible: bool, is_visible: bool) -> bool {
        !was_visible && is_visible
    }
}
