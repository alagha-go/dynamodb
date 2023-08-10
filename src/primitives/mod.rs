#![deny(dead_code)]
mod attribute_value;
mod attribute;
mod binary;
mod null;
mod attribute_error;
mod item;

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
#[doc(inline)]
pub use attribute::Attribute;
#[doc(inline)]
pub use attribute_error::AttributeError;
#[cfg(any(feature = "uuid", feature = "full"))]
use std::str::FromStr;
#[doc(inline)]
pub use binary::Binary;
#[doc(inline)]
pub use item::Item;
use std::rc::Rc;
use bytes::Bytes;
#[cfg(any(feature = "uuid", feature = "full"))]
use uuid::Uuid;


const BASE64: GeneralPurpose = general_purpose::STANDARD;