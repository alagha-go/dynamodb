use std::fmt::Display;
use std::error::Error;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum AttributeError {
    InvalidType,
    InvalidFormat,
    MissingField(&'static str)
}

impl Display for AttributeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AttributeError::InvalidType => write!(f, "invalid type"),
            AttributeError::InvalidFormat => write!(f, "invalid format"),
            AttributeError::MissingField(field) => write!(f, "missing field `{}`", field)
        }
    }
}

impl Error for AttributeError {}

unsafe impl Send for AttributeError {}

unsafe impl Sync for AttributeError {}