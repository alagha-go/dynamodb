mod attribute_value;
mod attribute;
mod binary;
mod null;

use std::collections::{HashMap, VecDeque};
use base64::engine::general_purpose;
use serde::{Serialize, Deserialize};
use base64::engine::GeneralPurpose;
use std::rc::Rc;
use attribute_value::*;
pub use binary::*;
pub use null::*;


pub type StdError = Box<dyn std::error::Error + Send + Sync + 'static>;
pub type Result<T> = std::result::Result<T, StdError>;


const BASE64: GeneralPurpose = general_purpose::STANDARD;