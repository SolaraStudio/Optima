use jni::objects::JObject;
use jni::JNIEnv;

pub struct JNICallbacks;

impl JNICallbacks {
    pub fn on_frame_ready(env: &JNIEnv, callback: JObject) {
        let class = env.find_class("org/optima/OptimaEngine").unwrap();
        let method = env.get_static_method_id(class, "onFrameReady", "()V").unwrap();
        let _ = env.call_static_method(class, method, &[]);
    }

    pub fn on_media_loaded(env: &JNIEnv, duration_ms: i64) {
        let class = env.find_class("org/optima/OptimaEngine").unwrap();
        let method = env.get_static_method_id(class, "onMediaLoaded", "(J)V").unwrap();
        let _ = env.call_static_method(class, method, &[(&duration_ms).into()]);
    }

    pub fn on_media_error(env: &JNIEnv, error: &str) {
        let class = env.find_class("org/optima/OptimaEngine").unwrap();
        let method = env.get_static_method_id(class, "onMediaError", "(Ljava/lang/String;)V").unwrap();
        let jstr = env.new_string(error).unwrap();
        let _ = env.call_static_method(class, method, &[(&jstr).into()]);
    }
}
