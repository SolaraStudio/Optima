use crate::css::colors::Color;
use crate::css::units::Length;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Keyword(String),
    String(String),
    Number(f32),
    Length(Length),
    Percentage(f32),
    Color(Color),
    Url(String),
    Function(String, Vec<Value>),
    List(Vec<Value>),
    None,
}

impl Value {
    pub fn is_none(&self) -> bool {
        matches!(self, Value::None)
    }

    pub fn as_number(&self) -> Option<f32> {
        match self {
            Value::Number(n) => Some(*n),
            _ => None,
        }
    }

    pub fn as_length(&self) -> Option<Length> {
        match self {
            Value::Length(l) => Some(*l),
            _ => None,
        }
    }

    pub fn as_color(&self) -> Option<Color> {
        match self {
            Value::Color(c) => Some(*c),
            _ => None,
        }
    }

    pub fn as_string(&self) -> Option<&str> {
        match self {
            Value::String(s) => Some(s),
            Value::Keyword(s) => Some(s),
            _ => None,
        }
    }
}
