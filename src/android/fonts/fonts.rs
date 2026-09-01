use crate::text::font_cache::FontCache;
use jni::objects::{JClass, JObject, JString};
use jni::JNIEnv;
use std::collections::HashMap;

pub struct AndroidFontHelper;

impl AndroidFontHelper {
    /// Retrieve system font paths from Kotlin.
    pub fn get_system_fonts(env: &mut JNIEnv) -> HashMap<String, String> {
        let class = env.find_class("org/optima/SystemFontHelper")
            .expect("Failed to find SystemFontHelper class");
        let method_id = env.get_static_method_id(class, "getSystemFonts", "()Ljava/util/Map;")
            .expect("Failed to find getSystemFonts method");
        let result = env.call_static_method(class, method_id, &[])
            .expect("Failed to call getSystemFonts");
        let map = result.l().expect("Result is not an object");

        // Convert Java Map to Rust HashMap
        let mut fonts = HashMap::new();
        let map_class = env.find_class("java/util/Map").unwrap();
        let entry_set_method = env.get_method_id(map_class, "entrySet", "()Ljava/util/Set;").unwrap();
        let entry_set = env.call_method(map, entry_set_method, &[]).unwrap().l().unwrap();

        let iterator_class = env.find_class("java/util/Iterator").unwrap();
        let has_next_method = env.get_method_id(iterator_class, "hasNext", "()Z").unwrap();
        let next_method = env.get_method_id(iterator_class, "next", "()Ljava/lang/Object;").unwrap();

        let iter = env.call_method(entry_set, "iterator", "()Ljava/util/Iterator;", &[]).unwrap().l().unwrap();

        while env.call_method(iter, has_next_method, &[]).unwrap().z().unwrap() {
            let entry = env.call_method(iter, next_method, &[]).unwrap().l().unwrap();
            let key = env.call_method(entry, "getKey", "()Ljava/lang/Object;", &[]).unwrap().l().unwrap();
            let value = env.call_method(entry, "getValue", "()Ljava/lang/Object;", &[]).unwrap().l().unwrap();

            let key_str: String = env.get_string(env.call_method(key, "toString", "()Ljava/lang/String;", &[]).unwrap().l().unwrap().into()).unwrap().into();
            let value_str: String = env.get_string(env.call_method(value, "toString", "()Ljava/lang/String;", &[]).unwrap().l().unwrap().into()).unwrap().into();
            fonts.insert(key_str, value_str);
        }

        fonts
    }

    /// Load all system fonts into the font cache.
    pub fn load_system_fonts(cache: &mut FontCache, env: &mut JNIEnv) {
        let fonts = Self::get_system_fonts(env);
        for (name, path) in fonts {
            if let Some(font) = cache.load_from_file(&name, &path) {
                log::info!("Loaded system font: {} from {}", name, path);
            } else {
                log::warn!("Failed to load system font: {} from {}", name, path);
            }
        }
    }
}
