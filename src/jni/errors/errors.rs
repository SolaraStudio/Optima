use jni::errors::Error;

pub type JNIResult<T> = Result<T, Error>;

pub fn map_jni_error(err: Error) -> String {
    match err {
        Error::NullPtr(_) => "Null pointer".to_string(),
        Error::JavaException => "Java exception".to_string(),
        Error::MethodNotFound => "Method not found".to_string(),
        Error::FieldNotFound => "Field not found".to_string(),
        Error::InvalidArgList => "Invalid argument list".to_string(),
        Error::NullField => "Null field".to_string(),
        Error::InvalidJValueConversion(_) => "Invalid JValue conversion".to_string(),
        Error::NullObject => "Null object".to_string(),
        Error::InvalidUtf8 => "Invalid UTF-8".to_string(),
        Error::JNIEnv(_) => "JNI environment error".to_string(),
        Error::Other(e) => format!("Other: {}", e),
    }
}
