use jni::errors::Error as JniError;
use std::fmt;
use std::io;

#[derive(Debug)]
pub enum OptimaError {
    Io(io::Error),
    Parse(String),
    Media(String),
    Network(String),
    Render(String),
    Jni(String),
    NotFound(String),
    InvalidState(String),
    Timeout(String),
    Permission(String),
    Unsupported(String),
}

impl fmt::Display for OptimaError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            OptimaError::Io(e) => write!(f, "IO error: {}", e),
            OptimaError::Parse(s) => write!(f, "Parse error: {}", s),
            OptimaError::Media(s) => write!(f, "Media error: {}", s),
            OptimaError::Network(s) => write!(f, "Network error: {}", s),
            OptimaError::Render(s) => write!(f, "Render error: {}", s),
            OptimaError::Jni(s) => write!(f, "JNI error: {}", s),
            OptimaError::NotFound(s) => write!(f, "Not found: {}", s),
            OptimaError::InvalidState(s) => write!(f, "Invalid state: {}", s),
            OptimaError::Timeout(s) => write!(f, "Timeout: {}", s),
            OptimaError::Permission(s) => write!(f, "Permission error: {}", s),
            OptimaError::Unsupported(s) => write!(f, "Unsupported: {}", s),
        }
    }
}

impl std::error::Error for OptimaError {}

pub type Result<T> = std::result::Result<T, OptimaError>;

impl From<io::Error> for OptimaError {
    fn from(err: io::Error) -> Self {
        OptimaError::Io(err)
    }
}

impl From<JniError> for OptimaError {
    fn from(err: JniError) -> Self {
        OptimaError::Jni(err.to_string())
    }
}

pub fn io_error(msg: &str) -> OptimaError {
    OptimaError::Io(io::Error::new(io::ErrorKind::Other, msg))
}

pub fn not_found(msg: &str) -> OptimaError {
    OptimaError::NotFound(msg.to_string())
}

pub fn invalid_state(msg: &str) -> OptimaError {
    OptimaError::InvalidState(msg.to_string())
}
