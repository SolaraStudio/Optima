pub struct JniConversions;

impl JniConversions {
    pub fn string_to_jstring(s: &str) -> Vec<u8> {
        s.as_bytes().to_vec()
    }
    pub fn jstring_to_string(bytes: &[u8]) -> Option<String> {
        String::from_utf8(bytes.to_vec()).ok()
    }
    pub fn int_to_jint(v: i32) -> i32 {
        v
    }
    pub fn float_to_jfloat(v: f32) -> f32 {
        v
    }
    pub fn bool_to_jboolean(v: bool) -> i32 {
        if v { 1 } else { 0 }
    }
}
