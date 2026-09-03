use jni::objects::{JClass, JString};
use jni::sys::jlong;
use jni::JNIEnv;

use crate::api::config::EngineConfig;
use crate::api::engine::Engine;

pub struct JniBridge;

impl JniBridge {
    pub fn new() -> Self {
        JniBridge
    }
}

impl Default for JniBridge {
    fn default() -> Self {
        Self::new()
    }
}

unsafe fn engine_ref(ptr: jlong) -> Option<&'static mut Engine> {
    if ptr == 0 {
        return None;
    }
    unsafe { Some(&mut *(ptr as *mut Engine)) }
}

fn get_string(env: &mut JNIEnv, s: &JString) -> String {
    env.get_string(s)
        .map(|js| js.into())
        .unwrap_or_default()
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_org_optima_OptimaEngine_nativeInit(
    _env: JNIEnv,
    _class: JClass,
) -> jlong {
    let engine = Box::new(Engine::new(EngineConfig::new()));
    Box::into_raw(engine) as jlong
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_org_optima_OptimaEngine_nativeLoadHtml(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    html: JString,
) {
    let input = get_string(&mut env, &html);
    if let Some(e) = unsafe { engine_ref(ptr) } {
        let _ = e.load_html(&input, "");
    }
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_org_optima_OptimaEngine_nativeLoadCss(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    css: JString,
) {
    let input = get_string(&mut env, &css);
    if let Some(e) = unsafe { engine_ref(ptr) } {
        let _ = e.inject_css(&input);
    }
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_org_optima_OptimaEngine_nativeRender(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) {
    if let Some(e) = unsafe { engine_ref(ptr) } {
        let _ = e.render();
    }
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_org_optima_OptimaEngine_nativeDestroy(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) {
    if ptr != 0 {
        unsafe {
            drop(Box::from_raw(ptr as *mut Engine));
        }
    }
}
