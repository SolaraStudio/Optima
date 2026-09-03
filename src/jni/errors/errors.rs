use thiserror::Error;

#[derive(Debug, Error)]
pub enum JniError {
    #[error("JNI call failed: {0}")]
    CallFailed(String),
    #[error("Invalid argument: {0}")]
    InvalidArgument(String),
    #[error("Type mismatch: expected {expected}, got {got}")]
    TypeMismatch { expected: String, got: String },
    #[error("Null pointer dereference")]
    NullPointer,
    #[error("Out of memory")]
    OutOfMemory,
}

impl JniError {
    pub fn call_failed(msg: &str) -> Self {
        JniError::CallFailed(msg.to_string())
    }
    pub fn invalid_arg(msg: &str) -> Self {
        JniError::InvalidArgument(msg.to_string())
    }
}
