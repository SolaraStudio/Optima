use jni::objects::{GlobalRef, JObject};
use jni::JNIEnv;

pub struct AndroidActivity {
    pub activity: GlobalRef,
}

impl AndroidActivity {
    pub fn new(env: &mut JNIEnv, activity: JObject) -> Self {
        let global_ref = env.new_global_ref(activity).unwrap();
        AndroidActivity {
            activity: global_ref,
        }
    }

    pub fn get_activity<'local>(&self, env: &mut JNIEnv<'local>) -> JObject<'local> {
        env.new_local_ref(&self.activity).unwrap()
    }

    pub fn get_application_context<'local>(env: &mut JNIEnv<'local>) -> JObject<'local> {
        let class = env.find_class("android/app/Activity").unwrap();
        env.call_static_method(
            class,
            "getApplicationContext",
            "()Landroid/content/Context;",
            &[],
        )
        .unwrap()
        .l()
        .unwrap()
    }
}
