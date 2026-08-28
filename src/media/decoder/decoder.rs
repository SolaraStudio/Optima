use jni::objects::{JObject, JClass};
use jni::JNIEnv;
use crate::media::{VideoFrame, AudioFrame};
use crate::media::codec::Codec;

pub struct Decoder {
    video_codec: Option<Codec>,
    audio_codec: Option<Codec>,
    video_handle: Option<jlong>,
    audio_handle: Option<jlong>,
    video_width: u32,
    video_height: u32,
    audio_sample_rate: u32,
    audio_channels: u8,
}

impl Decoder {
    pub fn new() -> Self {
        Self {
            video_codec: None,
            audio_codec: None,
            video_handle: None,
            audio_handle: None,
            video_width: 0,
            video_height: 0,
            audio_sample_rate: 0,
            audio_channels: 0,
        }
    }

    pub fn init_video_decoder(&mut self, env: &mut JNIEnv, codec: Codec, width: u32, height: u32) -> bool {
        let class = env.find_class("org/optima/MediaCodecDecoder").unwrap();
        let method = env.get_static_method_id(class, "create", "(Ljava/lang/String;II)J").unwrap();
        let mime_jstring = env.new_string(codec.mime_type()).unwrap();
        let result = env.call_static_method(class, method, &[
            (&mime_jstring).into(),
            width.into(),
            height.into(),
        ]).unwrap();
        let ptr = result.j().unwrap();
        self.video_handle = Some(ptr);
        self.video_codec = Some(codec);
        self.video_width = width;
        self.video_height = height;
        true
    }

    pub fn init_audio_decoder(&mut self, env: &mut JNIEnv, codec: Codec, sample_rate: u32, channels: u8) -> bool {
        let class = env.find_class("org/optima/MediaCodecDecoder").unwrap();
        let method = env.get_static_method_id(class, "createAudio", "(Ljava/lang/String;II)J").unwrap();
        let mime_jstring = env.new_string(codec.mime_type()).unwrap();
        let result = env.call_static_method(class, method, &[
            (&mime_jstring).into(),
            sample_rate.into(),
            channels.into(),
        ]).unwrap();
        let ptr = result.j().unwrap();
        self.audio_handle = Some(ptr);
        self.audio_codec = Some(codec);
        self.audio_sample_rate = sample_rate;
        self.audio_channels = channels;
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
            width: self.video_width,
            height: self.video_height,
            data: buffer,
            pts: 0,
            dts: 0,
            duration: 0,
            is_keyframe: true,
        })
    }

    pub fn decode_video_frame_with_pts(&self, env: &mut JNIEnv, data: &[u8], pts: u64) -> Option<VideoFrame> {
        let ptr = self.video_handle?;
        let class = env.find_class("org/optima/MediaCodecDecoder").unwrap();
        let method = env.get_method_id(class, "decodeFrame", "([B)[B").unwrap();
        let byte_array = env.byte_array_from_slice(data).unwrap();
        let result = env.call_method(ptr, method, &[(&byte_array).into()]).unwrap();
        let output = result.l().unwrap();
        let buffer = env.convert_byte_array(output).unwrap();
        Some(VideoFrame {
            width: self.video_width,
            height: self.video_height,
            data: buffer,
            pts,
            dts: 0,
            duration: 0,
            is_keyframe: true,
        })
    }

    pub fn decode_audio_frame(&self, data: &[u8]) -> Option<AudioFrame> {
        // Placeholder - implement with ffmpeg or MediaCodec
        None
    }

    pub fn has_video(&self) -> bool {
        self.video_handle.is_some()
    }

    pub fn has_audio(&self) -> bool {
        self.audio_handle.is_some()
    }

    pub fn get_video_codec(&self) -> Option<Codec> {
        self.video_codec
    }

    pub fn get_audio_codec(&self) -> Option<Codec> {
        self.audio_codec
    }

    pub fn get_video_width(&self) -> u32 {
        self.video_width
    }

    pub fn get_video_height(&self) -> u32 {
        self.video_height
    }

    pub fn get_audio_sample_rate(&self) -> u32 {
        self.audio_sample_rate
    }

    pub fn get_audio_channels(&self) -> u8 {
        self.audio_channels
    }
}

impl Default for Decoder {
    fn default() -> Self {
        Self::new()
    }
}
