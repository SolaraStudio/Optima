use jni::objects::JObject;
use jni::JNIEnv;

pub struct AndroidSurface {
    pub surface: JObject<'static>,
    pub width: u32,
    pub height: u32,
    pub format: u32,
    pub is_valid: bool,
}

impl AndroidSurface {
    pub fn new(env: &JNIEnv, surface: JObject, width: u32, height: u32) -> Self {
        let global = env.new_global_ref(surface).unwrap();
        Self {
            surface: global.into_inner(),
            width,
            height,
            format: 0,
            is_valid: true,
        }
    }

    pub fn from_java(env: &JNIEnv, surface: JObject) -> Self {
        Self {
            surface: env.new_global_ref(surface).unwrap().into_inner(),
            width: 0,
            height: 0,
            format: 0,
            is_valid: true,
        }
    }

    pub fn get_width(&self) -> u32 {
        self.width
    }

    pub fn get_height(&self) -> u32 {
        self.height
    }

    pub fn get_format(&self) -> u32 {
        self.format
    }

    pub fn is_valid(&self) -> bool {
        self.is_valid
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
    }

    pub fn set_format(&mut self, format: u32) {
        self.format = format;
    }

    pub fn invalidate(&mut self) {
        self.is_valid = false;
    }

    pub fn get_surface_ptr(&self) -> *const std::os::raw::c_void {
        self.surface as *const std::os::raw::c_void
    }

    pub fn to_jobject(&self, env: &JNIEnv) -> JObject {
        env.new_local_ref(self.surface).unwrap().into()
    }
}
