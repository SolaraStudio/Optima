use std::fmt;

#[derive(Debug)]
pub enum OptimaError {
    Io(std::io::Error),
    Parse(String),
    Media(String),
    Network(String),
    Render(String),
    Jni(String),
    NotFound(String),
    InvalidState(String),
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
        }
    }
}

impl std::error::Error for OptimaError {}

pub type Result<T> = std::result::Result<T, OptimaError>;
