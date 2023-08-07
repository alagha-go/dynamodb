#![allow(dead_code)]
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

impl From<std::num::ParseIntError> for AttributeError {
    fn from(_: std::num::ParseIntError) -> Self {
        AttributeError::InvalidFormat
    }
}

impl From<std::num::ParseFloatError> for AttributeError {
    fn from(_: std::num::ParseFloatError) -> Self {
        AttributeError::InvalidFormat
    }
}

#[cfg(any(feature = "bson", feature = "full"))]
impl From<bson::oid::Error> for AttributeError {
    fn from(_: bson::oid::Error) -> Self {
        AttributeError::InvalidFormat
    }
}

#[cfg(any(feature = "uuid", feature = "full"))]
impl From<String> for AttributeError {
    fn from(_: String) -> Self {
        AttributeError::InvalidFormat
    }
}

#[cfg(any(feature = "uuid", feature = "full"))]
impl From<uuid::Error> for AttributeError {
    fn from(_: uuid::Error) -> Self {
        AttributeError::InvalidFormat
    }
}

#[cfg(any(feature = "uuid", feature = "full"))]
impl From<chrono::format::ParseError> for AttributeError {
    fn from(_: chrono::format::ParseError) -> Self {
        AttributeError::InvalidFormat
    }
}