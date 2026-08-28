use jni::objects::JObject;
use jni::JNIEnv;

pub struct JNIHandle {
    pub ptr: *mut std::os::raw::c_void,
}

impl JNIHandle {
    pub fn new(ptr: *mut std::os::raw::c_void) -> Self {
        Self { ptr }
    }

    pub fn from_object(env: &JNIEnv, obj: JObject) -> Self {
        let ptr = env.new_global_ref(obj).unwrap().into_inner() as *mut std::os::raw::c_void;
        Self { ptr }
    }

    pub fn to_object(&self, env: &JNIEnv) -> JObject {
        env.new_local_ref(self.ptr as jobject).unwrap().into()
    }
}
