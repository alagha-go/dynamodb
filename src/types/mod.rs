#![allow(dead_code)]
mod attribute_value;
mod attribute;
mod binary;
mod null;
mod errors;

use std::collections::{HashMap, VecDeque, btree_map::BTreeMap, btree_set::BTreeSet, HashSet};
use base64::engine::general_purpose;
use serde::{Serialize, Deserialize};
use base64::engine::GeneralPurpose;
pub use attribute_value::*;
#[cfg(any(feature = "bson", feature = "full"))]
use bson::oid::ObjectId;
#[cfg(any(feature = "time", feature = "full"))]
use chrono::{Utc, DateTime, Local, FixedOffset};
#[cfg(any(feature = "time", feature = "full"))]
use duration_string::DurationString;
pub use attribute::*;
use std::str::FromStr;
pub use binary::*;
use std::rc::Rc;
pub use null::*;
use bytes::Bytes;
#[cfg(any(feature = "uuid", feature = "full"))]
use uuid::Uuid;


pub type StdError = Box<dyn std::error::Error + Send + Sync + 'static>;
pub type Result<T> = std::result::Result<T, StdError>;


const BASE64: GeneralPurpose = general_purpose::STANDARD;