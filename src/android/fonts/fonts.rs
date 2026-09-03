use jni::JNIEnv;
use jni::objects::JObject;

pub struct AndroidFonts;

impl AndroidFonts {
    pub fn get_system_fonts(_env: &mut JNIEnv) -> Vec<String> {
        vec![
            "sans-serif".to_string(),
            "serif".to_string(),
            "monospace".to_string(),
        ]
    }

    pub fn load_font<'local>(env: &mut JNIEnv<'local>, name: &str) -> JObject<'local> {
        let class = env.find_class("android/graphics/Typeface").unwrap();
        let jname = env.new_string(name).unwrap();
        env.call_static_method(
            class,
            "create",
            "(Ljava/lang/String;)Landroid/graphics/Typeface;",
            &[(&jname).into()],
        )
        .unwrap()
        .l()
        .unwrap()
    }
}
