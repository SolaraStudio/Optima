#[derive(Debug, Clone, PartialEq)]
pub enum JniType {
    Void,
    Boolean,
    Byte,
    Char,
    Short,
    Int,
    Long,
    Float,
    Double,
    Object(String),
    Array(Box<JniType>),
}

impl JniType {
    pub fn signature(&self) -> String {
        match self {
            JniType::Void => "V".to_string(),
            JniType::Boolean => "Z".to_string(),
            JniType::Byte => "B".to_string(),
            JniType::Char => "C".to_string(),
            JniType::Short => "S".to_string(),
            JniType::Int => "I".to_string(),
            JniType::Long => "J".to_string(),
            JniType::Float => "F".to_string(),
            JniType::Double => "D".to_string(),
            JniType::Object(name) => format!("L{};", name.replace('.', "/")),
            JniType::Array(inner) => format!("[{}", inner.signature()),
        }
    }
}
