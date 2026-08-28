use jni::objects::{JObject, JClass};
use jni::JNIEnv;
use crate::media::VideoFrame;
use crate::media::AudioFrame;

pub struct Decoder {
    video_codec: Option<String>,
    audio_codec: Option<String>,
    video_handle: Option<jlong>,
    audio_handle: Option<jlong>,
}

impl Decoder {
    pub fn new() -> Self {
        Self {
            video_codec: None,
            audio_codec: None,
            video_handle: None,
            audio_handle: None,
        }
    }

    pub fn init_video_decoder(&mut self, env: &mut JNIEnv, mime_type: &str) -> bool {
        let class = env.find_class("org/optima/MediaCodecDecoder").unwrap();
        let method = env.get_static_method_id(class, "create", "(Ljava/lang/String;)J").unwrap();
        let mime_jstring = env.new_string(mime_type).unwrap();
        let result = env.call_static_method(class, method, &[(&mime_jstring).into()]).unwrap();
        let ptr = result.j().unwrap();
        self.video_handle = Some(ptr);
        self.video_codec = Some(mime_type.to_string());
        true
    }

    pub fn decode_video_frame(&self, env: &mut JNIEnv, data: &[u8]) -> Option<VideoFrame> {
        let ptr = self.video_handle?;
        let class = env.find_class("org/optima/MediaCodecDecoder").unwrap();
        let method = env.get_method_id(class, "decodeFrame", "([B)[B").unwrap();
        let byte_array = env.byte_array_from_slice(data).unwrap();
        let result = env.call_method(ptr, method, &[(&byte_array).into()]).unwrap();
        let output = result.l().unwrap();
        let buffer = env.convert_byte_array(output).unwrap();
        Some(VideoFrame {
            width: 640,
            height: 480,
            data: buffer,
            pts: 0,
        })
    }

    pub fn decode_audio_frame(&self, _data: &[u8]) -> Option<AudioFrame> {
        None
    }
}
