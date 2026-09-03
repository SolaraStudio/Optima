use jni::JNIEnv;

pub struct AndroidDisplay {
    pub width: u32,
    pub height: u32,
    pub density: f32,
}

impl AndroidDisplay {
    pub fn get_metrics(env: &mut JNIEnv) -> Self {
        let class = env.find_class("android/util/DisplayMetrics").unwrap();
        let obj = env.new_object(class, "()V", &[]).unwrap();
        let width = env
            .get_field(&obj, "widthPixels", "I")
            .unwrap()
            .i()
            .unwrap() as u32;
        let height = env
            .get_field(&obj, "heightPixels", "I")
            .unwrap()
            .i()
            .unwrap() as u32;
        let density = env.get_field(&obj, "density", "F").unwrap().f().unwrap();
        AndroidDisplay {
            width,
            height,
            density,
        }
    }
}
