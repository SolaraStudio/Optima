use android_logger::Config;
use jni::objects::{JClass, JString};
use jni::sys::jlong;
use jni::JNIEnv;
use log::{info, LevelFilter};

struct OptimaEngine {
    // Placeholder – will hold Vello renderer, layout engine, etc.
}

#[no_mangle]
pub extern "C" fn Java_com_solara_optima_OptimaEngine_nativeInit(
    env: JNIEnv,
    _class: JClass,
) -> jlong {
    android_logger::init_once(Config::default().with_min_level(LevelFilter::Info));
    info!("Optima Engine initialized");
    let engine = Box::new(OptimaEngine {});
    Box::into_raw(engine) as jlong
}

#[no_mangle]
pub extern "C" fn Java_com_solara_optima_OptimaEngine_nativeLoadHtml(
    _env: JNIEnv,
    _class: JClass,
    _ptr: jlong,
    _html: JString,
) {
    info!("Loading HTML (will implement later)");
}

#[no_mangle]
pub extern "C" fn Java_com_solara_optima_OptimaEngine_nativeLoadCss(
    _env: JNIEnv,
    _class: JClass,
    _ptr: jlong,
    _css: JString,
) {
    info!("Loading CSS (will implement later)");
}

#[no_mangle]
pub extern "C" fn Java_com_solara_optima_OptimaEngine_nativeRender(
    _env: JNIEnv,
    _class: JClass,
    _ptr: jlong,
) {
    info!("Rendering (will implement later)");
}

#[no_mangle]
pub extern "C" fn Java_com_solara_optima_OptimaEngine_nativeDestroy(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) {
    if ptr != 0 {
        info!("Destroying Optima Engine");
        unsafe {
            drop(Box::from_raw(ptr as *mut OptimaEngine));
        }
    }
}
