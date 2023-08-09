use serde::{Deserializer, Serializer};
use serde::{Serialize, Deserialize};
use super::BASE64;
use base64::Engine;

/// holds a `Vec<u8>` that would be turned into the `B` AttributeValue of dynamodb
#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Binary(#[serde(with="self")] pub Vec<u8>);



#[doc(hidden)]
pub fn serialize<S: Serializer>(v: &Vec<u8>, s: S) -> Result<S::Ok, S::Error> {
    let base64 = BASE64.encode(v);
    String::serialize(&base64, s)
}

#[doc(hidden)]
pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<Vec<u8>, D::Error> {
    let base64 = String::deserialize(d)?;
    let bytes = BASE64.decode(base64.as_bytes()).map_err(|e| serde::de::Error::custom(e))?;
    Ok(bytes.into())
}