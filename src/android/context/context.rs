use jni::JNIEnv;
use jni::objects::{GlobalRef, JObject, JString};

pub struct AndroidContext {
    pub context: GlobalRef,
}

impl AndroidContext {
    pub fn new(env: &mut JNIEnv, context: JObject) -> Self {
        let global_ref = env.new_global_ref(context).unwrap();
        AndroidContext {
            context: global_ref,
        }
    }

    pub fn get_application_context<'local>(env: &mut JNIEnv<'local>) -> JObject<'local> {
        let class = env.find_class("android/app/Activity").unwrap();
        let activity = env
            .call_static_method(
                class,
                "getApplicationContext",
                "()Landroid/content/Context;",
                &[],
            )
            .unwrap();
        activity.l().unwrap()
    }

    pub fn get_package_name(env: &mut JNIEnv, context: JObject) -> String {
        let result = env
            .call_method(context, "getPackageName", "()Ljava/lang/String;", &[])
            .unwrap();
        let jstr = JString::from(result.l().unwrap());
        let cstr = env.get_string(&jstr).unwrap();
        cstr.into()
    }

    pub fn get_files_dir(env: &mut JNIEnv, context: JObject) -> String {
        let result = env
            .call_method(context, "getFilesDir", "()Ljava/io/File;", &[])
            .unwrap();
        let file = result.l().unwrap();
        Self::file_abs_path(env, file)
    }

    pub fn get_cache_dir(env: &mut JNIEnv, context: JObject) -> String {
        let result = env
            .call_method(context, "getCacheDir", "()Ljava/io/File;", &[])
            .unwrap();
        let file = result.l().unwrap();
        Self::file_abs_path(env, file)
    }

    fn file_abs_path(env: &mut JNIEnv, file: JObject) -> String {
        let path_result = env
            .call_method(file, "getAbsolutePath", "()Ljava/lang/String;", &[])
            .unwrap();
        let jstr = JString::from(path_result.l().unwrap());
        let cstr = env.get_string(&jstr).unwrap();
        cstr.into()
    }
}
