use jni::JNIEnv;
use jni::objects::{JClass, JObject, JString};
use jni::sys::{jboolean, jint, jlong, jstring};

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
    env.get_string(s).map(|js| js.into()).unwrap_or_default()
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
    _obj: JObject,
    ptr: jlong,
    html: JString,
) {
    let input = get_string(&mut env, &html);
    if let Some(e) = unsafe { engine_ref(ptr) } {
        let _ = e.load_html(&input, "");
    }
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_org_optima_OptimaEngine_nativeLoadUrl(
    mut env: JNIEnv,
    _obj: JObject,
    ptr: jlong,
    url: JString,
) {
    let input = get_string(&mut env, &url);
    if let Some(e) = unsafe { engine_ref(ptr) } {
        let _ = e.load_url(&input);
    }
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_org_optima_OptimaEngine_nativeLoadCss(
    mut env: JNIEnv,
    _obj: JObject,
    ptr: jlong,
    css: JString,
) {
    let input = get_string(&mut env, &css);
    if let Some(e) = unsafe { engine_ref(ptr) } {
        let _ = e.inject_css(&input);
    }
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_org_optima_OptimaEngine_nativeSetViewport(
    _env: JNIEnv,
    _obj: JObject,
    ptr: jlong,
    width: jint,
    height: jint,
) {
    if let Some(e) = unsafe { engine_ref(ptr) } {
        e.set_viewport(width.max(0) as u32, height.max(0) as u32);
    }
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_org_optima_OptimaEngine_nativeRender(
    _env: JNIEnv,
    _obj: JObject,
    ptr: jlong,
) {
    if let Some(e) = unsafe { engine_ref(ptr) } {
        let _ = e.render();
    }
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_org_optima_OptimaEngine_nativeReload(
    _env: JNIEnv,
    _obj: JObject,
    ptr: jlong,
) {
    if let Some(e) = unsafe { engine_ref(ptr) } {
        let _ = e.reload();
    }
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_org_optima_OptimaEngine_nativeGoBack(
    _env: JNIEnv,
    _obj: JObject,
    ptr: jlong,
) {
    if let Some(e) = unsafe { engine_ref(ptr) } {
        let _ = e.go_back();
    }
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_org_optima_OptimaEngine_nativeGoForward(
    _env: JNIEnv,
    _obj: JObject,
    ptr: jlong,
) {
    if let Some(e) = unsafe { engine_ref(ptr) } {
        let _ = e.go_forward();
    }
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_org_optima_OptimaEngine_nativeTick(
    _env: JNIEnv,
    _obj: JObject,
    ptr: jlong,
) {
    if let Some(e) = unsafe { engine_ref(ptr) } {
        e.tick();
    }
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_org_optima_OptimaEngine_nativeCallNativeHandler(
    mut env: JNIEnv,
    _obj: JObject,
    ptr: jlong,
    name: JString,
    arg: JString,
) -> jstring {
    let handler_name = get_string(&mut env, &name);
    let handler_arg = get_string(&mut env, &arg);
    let result = if let Some(e) = unsafe { engine_ref(ptr) } {
        e.call_native_handler(&handler_name, &handler_arg)
    } else {
        String::new()
    };
    env.new_string(result)
        .map(|js| js.into_raw())
        .unwrap_or(std::ptr::null_mut())
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_org_optima_OptimaEngine_nativeHasNativeHandler(
    mut env: JNIEnv,
    _obj: JObject,
    ptr: jlong,
    name: JString,
) -> jboolean {
    let handler_name = get_string(&mut env, &name);
    if let Some(e) = unsafe { engine_ref(ptr) } {
        e.has_native_handler(&handler_name) as jboolean
    } else {
        0
    }
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_org_optima_OptimaEngine_nativeRegisterAssetText(
    mut env: JNIEnv,
    _obj: JObject,
    ptr: jlong,
    path: JString,
    content_type: JString,
    text: JString,
) {
    let path_str = get_string(&mut env, &path);
    let ct_str = get_string(&mut env, &content_type);
    let text_str = get_string(&mut env, &text);
    if let Some(e) = unsafe { engine_ref(ptr) } {
        e.register_asset_text(&path_str, &ct_str, &text_str);
    }
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_org_optima_OptimaEngine_nativeHasLocalAsset(
    mut env: JNIEnv,
    _obj: JObject,
    ptr: jlong,
    path: JString,
) -> jboolean {
    let path_str = get_string(&mut env, &path);
    if let Some(e) = unsafe { engine_ref(ptr) } {
        e.has_local_asset(&path_str) as jboolean
    } else {
        0
    }
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_org_optima_OptimaEngine_nativeLocalAssetCount(
    _env: JNIEnv,
    _obj: JObject,
    ptr: jlong,
) -> jint {
    if let Some(e) = unsafe { engine_ref(ptr) } {
        e.local_asset_count() as jint
    } else {
        0
    }
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_org_optima_OptimaEngine_nativeHandlerNames(
    env: JNIEnv,
    _obj: JObject,
    ptr: jlong,
) -> jstring {
    let names = if let Some(e) = unsafe { engine_ref(ptr) } {
        e.handler_names().join(",")
    } else {
        String::new()
    };
    env.new_string(names)
        .map(|js| js.into_raw())
        .unwrap_or(std::ptr::null_mut())
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
