use jni::objects::{GlobalRef, JObject};
use jni::JNIEnv;

pub struct AndroidView {
    pub view: GlobalRef,
}

impl AndroidView {
    pub fn new(env: &mut JNIEnv, view: JObject) -> Self {
        let global_ref = env.new_global_ref(view).unwrap();
        AndroidView { view: global_ref }
    }

    pub fn get_view<'local>(&self, env: &mut JNIEnv<'local>) -> JObject<'local> {
        env.new_local_ref(&self.view).unwrap()
    }

    pub fn get_width(env: &mut JNIEnv, view: JObject) -> u32 {
        env.call_method(view, "getWidth", "()I", &[])
            .unwrap()
            .i()
            .unwrap() as u32
    }

    pub fn get_height(env: &mut JNIEnv, view: JObject) -> u32 {
        env.call_method(view, "getHeight", "()I", &[])
            .unwrap()
            .i()
            .unwrap() as u32
    }

    pub fn request_layout(env: &mut JNIEnv, view: JObject) {
        let _ = env.call_method(view, "requestLayout", "()V", &[]);
    }
}
