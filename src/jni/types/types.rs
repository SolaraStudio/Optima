use jni::objects::{JClass, JObject, JString};

pub type JNIContext = JNIEnv;
pub type JNIClass = JClass;
pub type JNIObject = JObject;
pub type JNIString = JString;
pub type JNIPtr = *mut std::os::raw::c_void;
