use serde::{Deserializer, Serializer};
use serde::{Serialize, Deserialize};
use super::{BASE64, Rc};
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

impl From<Vec<u8>> for Binary {
    fn from(value: Vec<u8>) -> Self {
        Binary(value)
    }
}

impl<const SIZE: usize> From<[u8; SIZE]> for Binary {
    fn from(value: [u8; SIZE]) -> Self {
        let value = value.to_vec();
        Binary(value)
    }
}

impl From<&[u8]> for Binary {
    fn from(value: &[u8]) -> Self {
        let value = value.into_iter().map(|byte|{*byte}).collect::<Vec<_>>();
        Binary(value)
    }
}

impl<T: Into<Binary>> From<Box<T>> for Binary {
    fn from(value: Box<T>) -> Self {
        value.into()
    }
}

impl<T: Into<Binary>> From<Rc<T>> for Binary {
    fn from(value: Rc<T>) -> Self {
        value.into()
    }
}