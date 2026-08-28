mod render;
mod media;
mod audio;

use android_logger::Config;
use jni::objects::{JClass, JObject};
use jni::sys::jlong;
use jni::JNIEnv;
use log::{info, LevelFilter};
use std::sync::Arc;
use wry::application::event_loop::EventLoop;
use wry::webview::WebViewBuilder;
use wry::webview::WebView;

// Optima engine state
struct OptimaEngine {
    renderer: render::VelloRenderer,
    media: media::MediaPipeline,
    audio: audio::AudioEngine,
}

#[no_mangle]
pub extern "C" fn Java_org_optima_OptimaEngine_nativeInit(
    env: JNIEnv,
    _class: JClass,
    surface: JObject,
) -> jlong {
    android_logger::init_once(Config::default().with_min_level(LevelFilter::Info));
    info!("Optima Engine initializing");

    // Convert Java Surface to a raw pointer (for wry)
    // In a real implementation, we'd use wry's Android handle
    let engine = Box::new(OptimaEngine {
        renderer: render::VelloRenderer::new(),
        media: media::MediaPipeline::new(),
        audio: audio::AudioEngine::new(),
    });

    Box::into_raw(engine) as jlong
}

#[no_mangle]
pub extern "C" fn Java_org_optima_OptimaEngine_nativeRenderFrame(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) {
    let engine = unsafe { &mut *(ptr as *mut OptimaEngine) };
    engine.renderer.render();
}

#[no_mangle]
pub extern "C" fn Java_org_optima_OptimaEngine_nativePlayAudio(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) {
    let engine = unsafe { &mut *(ptr as *mut OptimaEngine) };
    engine.audio.play_tone(440.0, 1.0);
}

#[no_mangle]
pub extern "C" fn Java_org_optima_OptimaEngine_nativeDestroy(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) {
    if ptr != 0 {
        info!("Destroying Optima Engine");
        unsafe { drop(Box::from_raw(ptr as *mut OptimaEngine)) };
    }
}
