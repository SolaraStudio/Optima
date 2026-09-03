use jni::JNIEnv;
use jni::objects::{GlobalRef, JObject};

pub struct AndroidSurface {
    pub surface: GlobalRef,
    pub width: u32,
    pub height: u32,
}

impl AndroidSurface {
    pub fn new(env: &mut JNIEnv, surface: JObject, width: u32, height: u32) -> Self {
        let global_ref = env.new_global_ref(surface).unwrap();
        AndroidSurface {
            surface: global_ref,
            width,
            height,
        }
    }

    pub fn from_java(env: &mut JNIEnv, surface: JObject) -> Self {
        let global_ref = env.new_global_ref(surface).unwrap();
        AndroidSurface {
            surface: global_ref,
            width: 0,
            height: 0,
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

    pub fn to_jobject<'local>(&self, env: &mut JNIEnv<'local>) -> JObject<'local> {
        env.new_local_ref(&self.surface).unwrap()
    }
}
