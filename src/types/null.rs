use serde::{Deserializer, Serializer};
use serde::{Serialize, Deserialize};


pub fn serialize<S: Serializer>(s: S) -> Result<S::Ok, S::Error> {
    bool::serialize(&true, s)
}

pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<(), D::Error> {
    bool::deserialize(d)?;
    Ok(())
}