use jni::objects::JObject;
use jni::JNIEnv;

pub struct AndroidPlatform;

impl AndroidPlatform {
    pub fn get_activity(env: &JNIEnv) -> JObject {
        let class = env.find_class("android/app/Activity").unwrap();
        let method = env.get_static_method_id(class, "getActivity", "()Landroid/app/Activity;").unwrap();
        env.call_static_method(class, method, &[]).unwrap().l().unwrap()
    }

    pub fn get_surface(env: &JNIEnv) -> JObject {
        let class = env.find_class("android/view/Surface").unwrap();
        let method = env.get_static_method_id(class, "getSurface", "()Landroid/view/Surface;").unwrap();
        env.call_static_method(class, method, &[]).unwrap().l().unwrap()
    }

    pub fn get_display_metrics(env: &JNIEnv) -> (u32, u32) {
        let class = env.find_class("android/util/DisplayMetrics").unwrap();
        let obj = env.new_object(class, "()V", &[]).unwrap();
        let width_field = env.get_field_id(class, "widthPixels", "I").unwrap();
        let height_field = env.get_field_id(class, "heightPixels", "I").unwrap();
        let width = env.get_field(obj, width_field).unwrap().i().unwrap() as u32;
        let height = env.get_field(obj, height_field).unwrap().i().unwrap() as u32;
        (width, height)
    }

    pub fn get_density(env: &JNIEnv) -> f32 {
        let class = env.find_class("android/util/DisplayMetrics").unwrap();
        let obj = env.new_object(class, "()V", &[]).unwrap();
        let field = env.get_field_id(class, "density", "F").unwrap();
        env.get_field(obj, field).unwrap().f().unwrap()
    }

    pub fn get_application_context(env: &JNIEnv) -> JObject {
        let class = env.find_class("android/app/Activity").unwrap();
        let method = env.get_static_method_id(class, "getApplicationContext", "()Landroid/content/Context;").unwrap();
        env.call_static_method(class, method, &[]).unwrap().l().unwrap()
    }
}
