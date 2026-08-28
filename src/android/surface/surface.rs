use jni::objects::JObject;
use jni::JNIEnv;

pub struct AndroidSurface {
    pub surface: JObject<'static>,
    pub width: u32,
    pub height: u32,
}

impl AndroidSurface {
    pub fn new(env: &JNIEnv, surface: JObject, width: u32, height: u32) -> Self {
        let global = env.new_global_ref(surface).unwrap();
        Self {
            surface: global.into_inner(),
            width,
            height,
        }
    }

    pub fn get_width(&self) -> u32 {
        self.width
    }

    pub fn get_height(&self) -> u32 {
        self.height
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
    }
}
