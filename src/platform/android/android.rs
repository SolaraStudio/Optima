pub struct AndroidPlatform {
    pub sdk_version: u32,
    pub package_name: String,
}

impl AndroidPlatform {
    pub fn new(sdk_version: u32, package_name: &str) -> Self {
        AndroidPlatform { sdk_version, package_name: package_name.to_string() }
    }

    pub fn is_supported(&self) -> bool { self.sdk_version >= 21 }
    pub fn has_webview(&self) -> bool { true }
}
