use jni::objects::{JObject, JString};
use jni::JNIEnv;
use std::ffi::CStr;
use std::os::raw::c_char;

pub fn jstring_to_string(env: &JNIEnv, jstr: JString) -> String {
    let c_str = env.get_string(jstr).unwrap();
    c_str.into()
}

pub fn string_to_jstring(env: &JNIEnv, s: &str) -> JString {
    env.new_string(s).unwrap()
}

pub fn jobject_to_raw(env: &JNIEnv, obj: JObject) -> *mut c_void {
    env.new_global_ref(obj).unwrap().into_inner() as *mut c_void
}
