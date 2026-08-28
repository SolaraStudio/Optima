use jni::objects::{JClass, JObject, JString};
use jni::sys::jlong;
use jni::JNIEnv;
use std::os::raw::c_void;
use std::sync::Mutex;

use crate::render::VelloRenderer;
use crate::media::MediaPipeline;
use crate::media::audio::AudioEngine;

pub struct OptimaEngine {
    pub renderer: VelloRenderer,
    pub media: MediaPipeline,
    pub audio: AudioEngine,
}

#[no_mangle]
pub extern "C" fn Java_org_optima_OptimaEngine_nativeInit(
    env: JNIEnv,
    _class: JClass,
    surface: JObject,
) -> jlong {
    android_logger::init_once(android_logger::Config::default().with_min_level(log::LevelFilter::Info));
    log::info!("Optima Engine initializing");

    let renderer = VelloRenderer::new(surface);
    let media = MediaPipeline::new();
    let audio = AudioEngine::new();

    let engine = Box::new(OptimaEngine {
        renderer,
        media,
        audio,
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
        log::info!("Destroying Optima Engine");
        unsafe { drop(Box::from_raw(ptr as *mut OptimaEngine)) };
    }
}
