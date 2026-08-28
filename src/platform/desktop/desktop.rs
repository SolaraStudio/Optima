pub struct DesktopPlatform;

impl DesktopPlatform {
    pub fn get_window_handle() -> *mut std::os::raw::c_void {
        std::ptr::null_mut()
    }

    pub fn get_screen_size() -> (u32, u32) {
        (1920, 1080)
    }

    pub fn get_dpi() -> f32 {
        96.0
    }
}
