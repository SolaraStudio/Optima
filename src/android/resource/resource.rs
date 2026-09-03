use crate::android::activity::AndroidActivity;
use jni::objects::{GlobalRef, JObject, JString, JValue};
use jni::JNIEnv;

pub struct AndroidResource {
    pub context: GlobalRef,
}

impl AndroidResource {
    pub fn new(env: &mut JNIEnv, context: JObject) -> Self {
        let global_ref = env.new_global_ref(context).unwrap();
        AndroidResource {
            context: global_ref,
        }
    }

    pub fn get_string(env: &mut JNIEnv, res_id: u32) -> String {
        let context = AndroidActivity::get_application_context(env);
        let result = env
            .call_method(context, "getString", "(I)Ljava/lang/String;", &[JValue::Int(res_id as i32)])
            .unwrap();
        let jstr = JString::from(result.l().unwrap());
        let cstr = env.get_string(&jstr).unwrap();
        cstr.into()
    }

    pub fn get_color(env: &mut JNIEnv, res_id: u32) -> u32 {
        let context = AndroidActivity::get_application_context(env);
        env.call_method(context, "getColor", "(I)I", &[JValue::Int(res_id as i32)])
            .unwrap()
            .i()
            .unwrap() as u32
    }

    pub fn get_drawable<'local>(env: &mut JNIEnv<'local>, res_id: u32) -> JObject<'local> {
        let context = AndroidActivity::get_application_context(env);
        env.call_method(
            context,
            "getDrawable",
            "(I)Landroid/graphics/drawable/Drawable;",
            &[JValue::Int(res_id as i32)],
        )
        .unwrap()
        .l()
        .unwrap()
    }

    pub fn open_raw_resource<'local>(env: &mut JNIEnv<'local>, res_id: u32) -> Option<JObject<'local>> {
        let context = AndroidActivity::get_application_context(env);
        let result = env
            .call_method(context, "getResources", "()Landroid/content/res/Resources;", &[])
            .unwrap();
        let resources = result.l().unwrap();
        let stream = env
            .call_method(resources, "openRawResource", "(I)Ljava/io/InputStream;", &[JValue::Int(res_id as i32)])
            .unwrap();
        stream.l().ok()
    }
}
