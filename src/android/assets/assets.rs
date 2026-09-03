use jni::JNIEnv;
use jni::objects::{GlobalRef, JObject, JObjectArray, JString};

pub struct AndroidAssets {
    pub asset_manager: GlobalRef,
}

impl AndroidAssets {
    pub fn new(env: &mut JNIEnv, asset_manager: JObject) -> Self {
        let global_ref = env.new_global_ref(asset_manager).unwrap();
        AndroidAssets {
            asset_manager: global_ref,
        }
    }

    pub fn from_context(env: &mut JNIEnv, context: JObject) -> Self {
        let asset_manager = env
            .call_method(
                context,
                "getAssets",
                "()Landroid/content/res/AssetManager;",
                &[],
            )
            .unwrap()
            .l()
            .unwrap();
        Self::new(env, asset_manager)
    }

    pub fn open_file(env: &mut JNIEnv, asset_manager: &GlobalRef, path: &str) -> Option<Vec<u8>> {
        let jpath = env.new_string(path).unwrap();
        let input_stream = env
            .call_method(
                asset_manager,
                "open",
                "(Ljava/lang/String;)Ljava/io/InputStream;",
                &[(&jpath).into()],
            )
            .unwrap();
        if let Ok(stream) = input_stream.l() {
            Self::read_stream(env, stream)
        } else {
            None
        }
    }

    pub fn read_stream(env: &mut JNIEnv, stream: JObject) -> Option<Vec<u8>> {
        let buffer = vec![0u8; 4096];
        let mut result = Vec::new();
        loop {
            let jbuffer = env.byte_array_from_slice(&buffer).unwrap();
            let bytes_read = env
                .call_method(&stream, "read", "([B)I", &[(&jbuffer).into()])
                .unwrap()
                .i()
                .unwrap();
            if bytes_read <= 0 {
                break;
            }
            let data = env.convert_byte_array(jbuffer).unwrap();
            result.extend_from_slice(&data[0..bytes_read as usize]);
        }
        Some(result)
    }

    pub fn list_files(env: &mut JNIEnv, asset_manager: &GlobalRef, path: &str) -> Vec<String> {
        let jpath = env.new_string(path).unwrap();
        let result = env
            .call_method(
                asset_manager,
                "list",
                "(Ljava/lang/String;)[Ljava/lang/String;",
                &[(&jpath).into()],
            )
            .unwrap();
        let array: JObjectArray = result.l().unwrap().into();
        let len = env.get_array_length(&array).unwrap();
        let mut files = Vec::new();
        for i in 0..len {
            let jstr: JObject = env.get_object_array_element(&array, i).unwrap();
            let jstring = JString::from(jstr);
            let cstr = env.get_string(&jstring).unwrap();
            files.push(cstr.into());
        }
        files
    }
}
