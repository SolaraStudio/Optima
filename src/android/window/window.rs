use jni::objects::{GlobalRef, JObject, JValue};
use jni::JNIEnv;

pub struct AndroidWindow {
    pub window: GlobalRef,
}

impl AndroidWindow {
    pub fn new(env: &mut JNIEnv, window: JObject) -> Self {
        let global_ref = env.new_global_ref(window).unwrap();
        AndroidWindow { window: global_ref }
    }

    pub fn get_window<'local>(&self, env: &mut JNIEnv<'local>) -> JObject<'local> {
        env.new_local_ref(&self.window).unwrap()
    }

    pub fn set_flags(env: &mut JNIEnv, window: JObject, flags: u32, mask: u32) {
        let _ = env.call_method(window, "setFlags", "(II)V", &[JValue::Int(flags as i32), JValue::Int(mask as i32)]);
    }

    pub fn add_flags(env: &mut JNIEnv, window: JObject, flags: u32) {
        let _ = env.call_method(window, "addFlags", "(I)V", &[JValue::Int(flags as i32)]);
    }

    pub fn clear_flags(env: &mut JNIEnv, window: JObject, flags: u32) {
        let _ = env.call_method(window, "clearFlags", "(I)V", &[JValue::Int(flags as i32)]);
    }
}
