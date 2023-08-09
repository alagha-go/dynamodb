use std::{time::{Duration, SystemTime}, borrow::Cow};
use super::*;


/// Trait that enables easy conversion from and to the dynamodb [AttributeValue] type
pub trait Attribute: Sized {
    /// conversion from a rust type into the [AttributeValue] type
    fn attribute(&self) -> AttributeValue;
    /// falliable conversion from the [AttributeValue] type
    fn value(value: AttributeValue) -> Result<Self, AttributeError>;
    /// conversion from a slice into the [AttributeValue]
    /// 
    /// `Note`: by default this function is implemented for you by converting it into the `L` attribute. if your type converts to `BS` or `NS` or `SS`
    /// then you have to implement it youself.
    fn vec_attribute(values: &[Self]) -> AttributeValue {
        AttributeValue::L(values.into_iter().map(|value| {value.attribute()}).collect::<Vec<AttributeValue>>())
    }

    /// falliable conversion from [`Option<AttributeValue>`]
    fn option_value(option: Option<AttributeValue>, field: & 'static str, default: Option<Self>) -> Result<Self, AttributeError> {
        match option {
            None => {
                match default {
                    None => Err(AttributeError::MissingField(field)),
                    Some(value) => Ok(value)
                }
            },
            Some(value) => Attribute::value(value)
        }
    }
}

fn value<T: Attribute>(attribute: AttributeValue) -> Result<T, AttributeError> {
    Attribute::value(attribute)
}


impl Attribute for u8 {
    fn attribute(&self) -> AttributeValue {
        AttributeValue::N(self.to_string())
    }

    fn value(attribute: AttributeValue) -> Result<Self, AttributeError> {
        match attribute {
            AttributeValue::N(number) => Ok(number.parse()?),
            _ => Err(AttributeError::InvalidType)
        }
    }

    fn vec_attribute(values: &[Self]) -> AttributeValue {
        AttributeValue::NS(values.into_iter().map(|value|{value.to_string()}).collect())
    }
}




impl Attribute for u16 {
    fn attribute(&self) -> AttributeValue {
        AttributeValue::N(self.to_string())
    }

    fn value(attribute: AttributeValue) -> Result<Self, AttributeError> {
        match attribute {
            AttributeValue::N(number) => Ok(number.parse()?),
            _ => Err(AttributeError::InvalidType)
        }
    }

    fn vec_attribute(values: &[Self]) -> AttributeValue {
        AttributeValue::NS(values.into_iter().map(|value|{value.to_string()}).collect())
    }

}


impl Attribute for u32 {
    fn attribute(&self) -> AttributeValue {
        AttributeValue::N(self.to_string())
    }

    fn value(attribute: AttributeValue) -> Result<Self, AttributeError> {
        match attribute {
            AttributeValue::N(number) => Ok(number.parse()?),
            _ => Err(AttributeError::InvalidType)
        }
    }

    fn vec_attribute(values: &[Self]) -> AttributeValue {
        AttributeValue::NS(values.into_iter().map(|value|{value.to_string()}).collect())
    }

}


impl Attribute for u64 {
    fn attribute(&self) -> AttributeValue {
        AttributeValue::N(self.to_string())
    }

    fn value(attribute: AttributeValue) -> Result<Self, AttributeError> {
        match attribute {
            AttributeValue::N(number) => Ok(number.parse()?),
            _ => Err(AttributeError::InvalidType)
        }
    }

    fn vec_attribute(values: &[Self]) -> AttributeValue {
        AttributeValue::NS(values.into_iter().map(|value|{value.to_string()}).collect())
    }

}


impl Attribute for usize {
    fn attribute(&self) -> AttributeValue {
        AttributeValue::N(self.to_string())
    }

    fn value(attribute: AttributeValue) -> Result<Self, AttributeError> {
        match attribute {
            AttributeValue::N(number) => Ok(number.parse()?),
            _ => Err(AttributeError::InvalidType)
        }
    }

    fn vec_attribute(values: &[Self]) -> AttributeValue {
        AttributeValue::NS(values.into_iter().map(|value|{value.to_string()}).collect())
    }

}


impl Attribute for u128 {
    fn attribute(&self) -> AttributeValue {
        AttributeValue::N(self.to_string())
    }

    fn value(attribute: AttributeValue) -> Result<Self, AttributeError> {
        match attribute {
            AttributeValue::N(number) => Ok(number.parse()?),
            _ => Err(AttributeError::InvalidType)
        }
    }

    fn vec_attribute(values: &[Self]) -> AttributeValue {
        AttributeValue::NS(values.into_iter().map(|value|{value.to_string()}).collect())
    }

}


impl Attribute for i8 {
    fn attribute(&self) -> AttributeValue {
        AttributeValue::N(self.to_string())
    }

    fn value(attribute: AttributeValue) -> Result<Self, AttributeError> {
        match attribute {
            AttributeValue::N(number) => Ok(number.parse()?),
            _ => Err(AttributeError::InvalidType)
        }
    }

    fn vec_attribute(values: &[Self]) -> AttributeValue {
        AttributeValue::NS(values.into_iter().map(|value|{value.to_string()}).collect())
    }

}


impl Attribute for i16 {
    fn attribute(&self) -> AttributeValue {
        AttributeValue::N(self.to_string())
    }

    fn value(attribute: AttributeValue) -> Result<Self, AttributeError> {
        match attribute {
            AttributeValue::N(number) => Ok(number.parse()?),
            _ => Err(AttributeError::InvalidType)
        }
    }

    fn vec_attribute(values: &[Self]) -> AttributeValue {
        AttributeValue::NS(values.into_iter().map(|value|{value.to_string()}).collect())
    }

}


impl Attribute for i32 {
    fn attribute(&self) -> AttributeValue {
        AttributeValue::N(self.to_string())
    }

    fn value(attribute: AttributeValue) -> Result<Self, AttributeError> {
        match attribute {
            AttributeValue::N(number) => Ok(number.parse()?),
            _ => Err(AttributeError::InvalidType)
        }
    }

    fn vec_attribute(values: &[Self]) -> AttributeValue {
        AttributeValue::NS(values.into_iter().map(|value|{value.to_string()}).collect())
    }

}


impl Attribute for i64 {
    fn attribute(&self) -> AttributeValue {
        AttributeValue::N(self.to_string())
    }

    fn value(attribute: AttributeValue) -> Result<Self, AttributeError> {
        match attribute {
            AttributeValue::N(number) => Ok(number.parse()?),
            _ => Err(AttributeError::InvalidType)
        }
    }

    fn vec_attribute(values: &[Self]) -> AttributeValue {
        AttributeValue::NS(values.into_iter().map(|value|{value.to_string()}).collect())
    }

}


impl Attribute for isize {
    fn attribute(&self) -> AttributeValue {
        AttributeValue::N(self.to_string())
    }

    fn value(attribute: AttributeValue) -> Result<Self, AttributeError> {
        match attribute {
            AttributeValue::N(number) => Ok(number.parse()?),
            _ => Err(AttributeError::InvalidType)
        }
    }

    fn vec_attribute(values: &[Self]) -> AttributeValue {
        AttributeValue::NS(values.into_iter().map(|value|{value.to_string()}).collect())
    }

}


impl Attribute for i128 {
    fn attribute(&self) -> AttributeValue {
        AttributeValue::N(self.to_string())
    }

    fn value(attribute: AttributeValue) -> Result<Self, AttributeError> {
        match attribute {
            AttributeValue::N(number) => Ok(number.parse()?),
            _ => Err(AttributeError::InvalidType)
        }
    }

    fn vec_attribute(values: &[Self]) -> AttributeValue {
        AttributeValue::NS(values.into_iter().map(|value|{value.to_string()}).collect())
    }

}


impl Attribute for f32 {
    fn attribute(&self) -> AttributeValue {
        AttributeValue::N(self.to_string())
    }

    fn value(attribute: AttributeValue) -> Result<Self, AttributeError> {
        match attribute {
            AttributeValue::N(number) => Ok(number.parse()?),
            _ => Err(AttributeError::InvalidType)
        }
    }

    fn vec_attribute(values: &[Self]) -> AttributeValue {
        AttributeValue::NS(values.into_iter().map(|value|{value.to_string()}).collect())
    }

}


impl Attribute for f64 {
    fn attribute(&self) -> AttributeValue {
        AttributeValue::N(self.to_string())
    }

    fn value(attribute: AttributeValue) -> Result<Self, AttributeError> {
        match attribute {
            AttributeValue::N(number) => Ok(number.parse()?),
            _ => Err(AttributeError::InvalidType)
        }
    }

    fn vec_attribute(values: &[Self]) -> AttributeValue {
        AttributeValue::NS(values.into_iter().map(|value|{value.to_string()}).collect())
    }

}


impl Attribute for bool {
    fn attribute(&self) -> AttributeValue {
        AttributeValue::BOOL(*self)
    }

    fn value(attribute: AttributeValue) -> Result<Self, AttributeError> {
        match attribute {
            AttributeValue::BOOL(b) => Ok(b),
            _ => Err(AttributeError::InvalidType)
        }
    }
}


impl Attribute for String {
    fn attribute(&self) -> AttributeValue {
        AttributeValue::S(self.to_owned())
    }

    fn value(attribute: AttributeValue) -> Result<Self, AttributeError> {
        match attribute {
            AttributeValue::S(string) => Ok(string),
            AttributeValue::N(string) => Ok(string),
            _ => Err(AttributeError::InvalidType)
        }
    }

    fn vec_attribute(values: &[Self]) -> AttributeValue {
        AttributeValue::SS(values.into_iter().map(|value| {value.to_string()}).collect())
    }

}


impl Attribute for Cow<'_, str> {
    fn attribute(&self) -> AttributeValue {
        AttributeValue::S(self.to_string())
    }

    fn value(value: AttributeValue) -> Result<Self, AttributeError> {
        match value {
            AttributeValue::S(string) => Ok(Cow::Owned(string)),
            _ => Err(AttributeError::InvalidType)
        }
    }
}


impl<V: Attribute> Attribute for Option<V> {
    fn attribute(&self) -> AttributeValue {
        match self {
            None => AttributeValue::None,
            Some(value) => value.attribute()
        }
    }

    fn value(attribute: AttributeValue) -> Result<Self, AttributeError> {
        match attribute {
            AttributeValue::None => Ok(None),
            _ => Ok(Some(Attribute::value(attribute)?))
        }
    }
}


/// Attribute implementation for any type that implements the [Attribute] trait
impl<V: Attribute + Clone> Attribute for Vec<V> {
    /// uses the [Attribute::vec_attribute] function to do the conversion
    fn attribute(&self) -> AttributeValue {
        V::vec_attribute(self)
    }

    /// does the conversion from either the `L` or `BS` or `NS` or `SS` Attributes
    fn value(attribute: AttributeValue) -> Result<Self, AttributeError> {
        match attribute {
            AttributeValue::L(list) => {
                let mut values = Vec::new();
                for attribute in list {
                    values.push(Attribute::value(attribute)?)
                }
                Ok(values)
            }
            AttributeValue::SS(strings) => {
                let mut values = Vec::new();
                for string in strings {
                    values.push(Attribute::value(AttributeValue::S(string))?)
                }
                Ok(values)
            }
            AttributeValue::BS(binaries) => {
                let mut values = Vec::new();
                for binary in binaries {
                    values.push(Attribute::value(AttributeValue::B(binary))?)
                }
                Ok(values)
            }
            AttributeValue::NS(numbers) => {
                let mut values = Vec::new();
                for number in numbers {
                    values.push(Attribute::value(AttributeValue::N(number))?)
                }
                Ok(values)
            }
            _ => Err(AttributeError::InvalidType)
        }
    }
}


impl Attribute for Bytes {
    fn attribute(&self) -> AttributeValue {
        AttributeValue::B(Binary(self.to_vec()))
    }

    fn value(value: AttributeValue) -> Result<Self, AttributeError> {
        match value {
            AttributeValue::B(binary) => Ok(Bytes::from(binary.0)),
            _ => Err(AttributeError::InvalidType)
        }
    }

    fn vec_attribute(values: &[Self]) -> AttributeValue {
        AttributeValue::BS(values.into_iter().map(|value|{Binary(value.to_vec())}).collect())
    }
}


impl Attribute for Vec<Binary> {
    fn attribute(&self) -> AttributeValue {
        AttributeValue::BS(self.clone())
    }

    fn value(attribute: AttributeValue) -> Result<Self, AttributeError> {
        match attribute {
            AttributeValue::BS(binaries) => Ok(binaries),
            _ => Err(AttributeError::InvalidType)
        }
    }
}


/// implements the conversion just as `Vec<V>`
impl<V: Attribute + Clone + Ord> Attribute for BTreeSet<V> {
    fn attribute(&self) -> AttributeValue {
        let values: Vec<V> = self.into_iter().map(|value|{value.clone()}).collect();
        let values = values.as_slice();
        V::vec_attribute(values)
    }

    fn value(value: AttributeValue) -> Result<Self, AttributeError> {
        match value {
            AttributeValue::L(list) => {
                let mut tree = BTreeSet::new();
                for value in list {
                    tree.insert(Attribute::value(value)?);
                }
                Ok(tree)
            },
            AttributeValue::BS(binaries) => {
                let mut tree = BTreeSet::new();
                for binary in binaries {
                    tree.insert(Attribute::value(AttributeValue::B(binary))?);
                }
                Ok(tree)
            },
            AttributeValue::NS(numbers) => {
                let mut tree = BTreeSet::new();
                for number in numbers {
                    tree.insert(Attribute::value(AttributeValue::N(number))?);
                }
                Ok(tree)
            },
            AttributeValue::SS(strings) => {
                let mut tree = BTreeSet::new();
                for string in strings {
                    tree.insert(Attribute::value(AttributeValue::S(string))?);
                }
                Ok(tree)
            },
            _ => Err(AttributeError::InvalidType)?
        }
    }
}

/// implements the conversion just as `Vec<V>`
impl<V: Attribute + Clone + std::hash::Hash + Eq> Attribute for HashSet<V> {
    fn attribute(&self) -> AttributeValue {
        let values: Vec<V> = self.into_iter().map(|value|{value.clone()}).collect();
        V::vec_attribute(&values)
    }

    fn value(value: AttributeValue) -> Result<Self, AttributeError> {
        match value {
            AttributeValue::L(list) => {
                let mut set = HashSet::new();
                for value in list {
                    set.insert(Attribute::value(value)?);
                }
                Ok(set)
            },
            AttributeValue::BS(binaries) => {
                let mut set = HashSet::new();
                for binary in binaries {
                    set.insert(Attribute::value(AttributeValue::B(binary))?);
                }
                Ok(set)
            },
            AttributeValue::NS(numbers) => {
                let mut set = HashSet::new();
                for number in numbers {
                    set.insert(Attribute::value(AttributeValue::N(number))?);
                }
                Ok(set)
            },
            AttributeValue::SS(strings) => {
                let mut set = HashSet::new();
                for string in strings {
                    set.insert(Attribute::value(AttributeValue::S(string))?);
                }
                Ok(set)
            },
            _ => Err(AttributeError::InvalidType)?
        }
    }
}

/// A Map type for Items for HashMaps, represented as the `M` AttributeValue type
impl<K: From<String> + Into<String> + Clone + Eq + std::hash::Hash, V: Attribute + Clone> Attribute for HashMap<K, V> {
    fn attribute(&self) -> AttributeValue {
        let mut map = HashMap::new();
        let _ = self.into_iter().map(|(key, value)|{map.insert(key.clone().into(), value.attribute())});
        AttributeValue::M(map)
    }

    fn value(attribute: AttributeValue) -> Result<Self, AttributeError> {
        match attribute {
            AttributeValue::M(object) => {
                let mut map = HashMap::new();
                for (key, value) in object {
                    map.insert(key.into(), V::value(value)?);
                }
                Ok(map)
            },
            _ => Err(AttributeError::InvalidType)
        }
    }
}

/// A Map type for `Items` for `BTreeMaps`, represented as the `M` AttributeValue type
impl<K: From<String> + Into<String> + Clone + Ord + Eq + std::hash::Hash, V: Attribute + Clone> Attribute for BTreeMap<K, V> {
    fn attribute(&self) -> AttributeValue {
        let mut map = HashMap::new();
        let _ = self.into_iter().map(|(key, value)|{map.insert(key.clone().into(), value.attribute())});
        AttributeValue::M(map)
    }

    fn value(attribute: AttributeValue) -> Result<Self, AttributeError> {
        match attribute {
            AttributeValue::M(object) => {
                let mut tree = BTreeMap::new();
                for (key, value) in object {
                    tree.insert(key.into(), V::value(value)?);
                }
                Ok(tree)
            },
            _ => Err(AttributeError::InvalidType)
        }
    }
}


impl<A: Attribute> Attribute for (A,) {
    fn attribute(&self) -> AttributeValue {
        self.0.attribute()
    }

    fn value(attribute: AttributeValue) -> Result<Self, AttributeError> {
        Ok((A::value(attribute)?,))
    }
}


impl<A: Attribute, B: Attribute> Attribute for (A, B) {
    fn attribute(&self) -> AttributeValue {
        AttributeValue::L(vec![self.0.attribute(), self.1.attribute()])
    }

    fn value(attribute: AttributeValue) -> Result<Self, AttributeError> {
        match attribute {
            AttributeValue::L(mut list) => {
                Ok((Attribute::value(list.remove(0))?, Attribute::value(list.remove(0))?))
            },
            _ => Err(AttributeError::InvalidType)
        }
    }
}


impl<A: Attribute, B: Attribute, C: Attribute> Attribute for (A, B, C) {
    fn attribute(&self) -> AttributeValue {
        AttributeValue::L(vec![self.0.attribute(), self.1.attribute(), self.2.attribute()])
    }

    fn value(attribute: AttributeValue) -> Result<Self, AttributeError> {
        match attribute {
            AttributeValue::L(mut list) => {
                Ok((Attribute::value(list.remove(0))?, Attribute::value(list.remove(0))?, Attribute::value(list.remove(0))?))
            },
            _ => Err(AttributeError::InvalidType)
        }
    }
}


impl<A: Attribute, B: Attribute, C: Attribute, D: Attribute> Attribute for (A, B, C, D) {
    fn attribute(&self) -> AttributeValue {
        AttributeValue::L(vec![self.0.attribute(), self.1.attribute(), self.2.attribute(), self.3.attribute()])
    }

    fn value(attribute: AttributeValue) -> Result<Self, AttributeError> {
        match attribute {
            AttributeValue::L(mut list) => {
                Ok((Attribute::value(list.remove(0))?, Attribute::value(list.remove(0))?, Attribute::value(list.remove(0))?, Attribute::value(list.remove(0))?))
            },
            _ => Err(AttributeError::InvalidType)
        }
    }
}


impl<A: Attribute, B: Attribute, C: Attribute, D: Attribute, E: Attribute> Attribute for (A, B, C, D, E) {
    fn attribute(&self) -> AttributeValue {
        AttributeValue::L(vec![self.0.attribute(), self.1.attribute(), self.2.attribute(), self.3.attribute(), self.4.attribute()])
    }

    fn value(attribute: AttributeValue) -> Result<Self, AttributeError> {
        match attribute {
            AttributeValue::L(mut list) => {
                Ok((Attribute::value(list.remove(0))?, Attribute::value(list.remove(0))?, Attribute::value(list.remove(0))?, Attribute::value(list.remove(0))?, Attribute::value(list.remove(0))?))
            },
            _ => Err(AttributeError::InvalidType)
        }
    }
}


impl<A: Attribute, B: Attribute, C: Attribute, D: Attribute, E: Attribute, F: Attribute> Attribute for (A, B, C, D, E, F) {
    fn attribute(&self) -> AttributeValue {
        AttributeValue::L(vec![self.0.attribute(), self.1.attribute(), self.2.attribute(), self.3.attribute(), self.4.attribute(), self.5.attribute()])
    }

    fn value(attribute: AttributeValue) -> Result<Self, AttributeError> {
        match attribute {
            AttributeValue::L(mut list) => {
                Ok((value(list.remove(0))?, value(list.remove(0))?, value(list.remove(0))?, value(list.remove(0))?, value(list.remove(0))?, value(list.remove(0))?))
            },
            _ => Err(AttributeError::InvalidType)
        }
    }
}


impl<A: Attribute, B: Attribute, C: Attribute, D: Attribute, E: Attribute, F: Attribute, G: Attribute> Attribute for (A, B, C, D, E, F, G) {
    fn attribute(&self) -> AttributeValue {
        AttributeValue::L(vec![self.0.attribute(), self.1.attribute(), self.2.attribute(), self.3.attribute(), self.4.attribute(), self.5.attribute(), self.6.attribute()])
    }

    fn value(attribute: AttributeValue) -> Result<Self, AttributeError> {
        match attribute {
            AttributeValue::L(mut list) => {
                Ok((value(list.remove(0))?, value(list.remove(0))?, value(list.remove(0))?, value(list.remove(0))?, value(list.remove(0))?, value(list.remove(0))?, value(list.remove(0))?))
            },
            _ => Err(AttributeError::InvalidType)
        }
    }
}


impl<A: Attribute, B: Attribute, C: Attribute, D: Attribute, E: Attribute, F: Attribute, G: Attribute, H: Attribute> Attribute for (A, B, C, D, E, F, G, H) {
    fn attribute(&self) -> AttributeValue {
        AttributeValue::L(vec![self.0.attribute(), self.1.attribute(), self.2.attribute(), self.3.attribute(), self.4.attribute(), self.5.attribute(), self.6.attribute(), self.7.attribute()])
    }

    fn value(attribute: AttributeValue) -> Result<Self, AttributeError> {
        match attribute {
            AttributeValue::L(mut list) => {
                Ok((value(list.remove(0))?, value(list.remove(0))?, value(list.remove(0))?, value(list.remove(0))?, value(list.remove(0))?, value(list.remove(0))?, value(list.remove(0))?, value(list.remove(0))?))
            },
            _ => Err(AttributeError::InvalidType)
        }
    }
}


impl<A: Attribute, B: Attribute, C: Attribute, D: Attribute, E: Attribute, F: Attribute, G: Attribute, H: Attribute, I: Attribute> Attribute for (A, B, C, D, E, F, G, H, I) {
    fn attribute(&self) -> AttributeValue {
        AttributeValue::L(vec![self.0.attribute(), self.1.attribute(), self.2.attribute(), self.3.attribute(), self.4.attribute(), self.5.attribute(), self.6.attribute(), self.7.attribute(), self.8.attribute()])
    }

    fn value(attribute: AttributeValue) -> Result<Self, AttributeError> {
        match attribute {
            AttributeValue::L(mut list) => {
                Ok((value(list.remove(0))?, value(list.remove(0))?, value(list.remove(0))?, value(list.remove(0))?, value(list.remove(0))?, value(list.remove(0))?, value(list.remove(0))?, value(list.remove(0))?, value(list.remove(0))?))
            },
            _ => Err(AttributeError::InvalidType)
        }
    }
}


impl<A: Attribute, B: Attribute, C: Attribute, D: Attribute, E: Attribute, F: Attribute, G: Attribute, H: Attribute, I: Attribute, J: Attribute> Attribute for (A, B, C, D, E, F, G, H, I, J) {
    fn attribute(&self) -> AttributeValue {
        AttributeValue::L(vec![self.0.attribute(), self.1.attribute(), self.2.attribute(), self.3.attribute(), self.4.attribute(), self.5.attribute(), self.6.attribute(), self.7.attribute(), self.8.attribute(), self.9.attribute()])
    }

    fn value(attribute: AttributeValue) -> Result<Self, AttributeError> {
        match attribute {
            AttributeValue::L(mut list) => {
                Ok((value(list.remove(0))?, value(list.remove(0))?, value(list.remove(0))?, value(list.remove(0))?, value(list.remove(0))?, value(list.remove(0))?, value(list.remove(0))?, value(list.remove(0))?, value(list.remove(0))?, value(list.remove(0))?))
            },
            _ => Err(AttributeError::InvalidType)
        }
    }
}


impl <T: Attribute + Default, const SIZE: usize> Attribute for [T; SIZE] {
    fn attribute(&self) -> AttributeValue {
        Attribute::vec_attribute(self)
    }

    fn value(attribute: AttributeValue) -> Result<Self, AttributeError> {
        use AttributeValue::*;
        match attribute {
            L(mut list)  => {
                let mut vec = Vec::new();
                for _ in 0..SIZE {
                    vec.push(Attribute::value(list.remove(0))?)
                }
                let result = vec.try_into();
                match result {
                    Ok(array) => Ok(array),
                    Err(_) => Err(AttributeError::InvalidType)
                }
            }
            BS(mut binaries) => {
                let mut vec = Vec::new();
                for _ in 0..SIZE {
                    vec.push(Attribute::value(B(binaries.remove(0)))?)
                }
                let result = vec.try_into();
                match result {
                    Ok(array) => Ok(array),
                    Err(_) => Err(AttributeError::InvalidType)
                }
            }
            SS(mut strings) => {
                let mut vec = Vec::new();
                for _ in 0..SIZE {
                    vec.push(Attribute::value(S(strings.remove(0)))?)
                }
                let result = vec.try_into();
                match result {
                    Ok(array) => Ok(array),
                    Err(_) => Err(AttributeError::InvalidType)
                }
            }
            NS(mut numbers) => {
                let mut vec = Vec::new();
                for _ in 0..SIZE {
                    vec.push(Attribute::value(N(numbers.remove(0)))?)
                }
                let result = vec.try_into();
                match result {
                    Ok(array) => Ok(array),
                    Err(_) => Err(AttributeError::InvalidType)
                }
            }
            _ => Err(AttributeError::InvalidType)
        }
    }
}

/// implements the conversion using the `Vec<V>`
impl<T: Attribute + Clone> Attribute for VecDeque<T> {
    fn attribute(&self) -> AttributeValue {
        let values = Into::<Vec<T>>::into(self.clone());
        T::vec_attribute(&values)
    }

    fn value(value: AttributeValue) -> Result<Self, AttributeError> {
        let array: Vec<T> = Attribute::value(value)?;
        Ok(array.into())
    }
}


impl<T: Attribute> Attribute for Box<T> {
    fn attribute(&self) -> AttributeValue {
        let value = &**self;
        Attribute::attribute(value)
    }

    fn value(value: AttributeValue) -> Result<Self, AttributeError> {
        let value = Attribute::value(value)?;
        Ok(Box::new(value))
    }
}


impl<T: Attribute + Clone> Attribute for Box<[T]> {
    fn attribute(&self) -> AttributeValue {
        let value = &**self;
        let values = value.to_vec();
        values.attribute()
    }

    fn value(value: AttributeValue) -> Result<Self, AttributeError> {
        let list = Vec::<T>::value(value)?;
        Ok(list.into())
    }
}


impl<T: Attribute> Attribute for Rc<T> {
    fn attribute(&self) -> AttributeValue {
        let value = &**self;
        Attribute::attribute(value)
    }

    fn value(value: AttributeValue) -> Result<Self, AttributeError> {
        let value = T::value(value)?;
        Ok(Rc::new(value))
    }
}


impl<T: Attribute + Clone> Attribute for Rc<[T]> {
    fn attribute(&self) -> AttributeValue {
        let value = &**self;
        let values = value.to_vec();
        values.attribute()
    }

    fn value(value: AttributeValue) -> Result<Self, AttributeError> {
        let list = Vec::<T>::value(value)?;
        Ok(list.into())
    }
}


/// A `String` type for `Uuids`, represented by the `S` AttributeValue type
#[cfg(any(feature = "uuid", feature = "full"))]
impl Attribute for Uuid {
    fn attribute(&self) -> AttributeValue {
        AttributeValue::S(self.to_string())
    }

    fn value(value: AttributeValue) -> Result<Self, AttributeError> {
        match value {
            AttributeValue::S(value) => Ok(Uuid::parse_str(&value)?),
            AttributeValue::B(binary) => Ok(Uuid::from_slice(binary.0.as_ref())?),
            _ => Err(AttributeError::InvalidType)
        }
    }
}

/// A hex version of the mongodb`ObjectId`, represented by the `S` AttributeValue type
#[cfg(any(feature = "bson", feature = "full"))]
impl Attribute for ObjectId {
    fn attribute(&self) -> AttributeValue {
        AttributeValue::S(self.to_hex())
    }

    /// converts from either the `S` or the `B` attributes
    fn value(value: AttributeValue) -> Result<Self, AttributeError> {
        match value {
            AttributeValue::S(value) => Ok(Self::from_str(&value)?),
            AttributeValue::B(binary) => {
                let bytes = match TryInto::<[u8; 12]>::try_into(binary.0) {
                    Ok(bytes) => bytes,
                    Err(_) => return  Err(AttributeError::InvalidType)
                };
                Ok(Self::from_bytes(bytes))
            },
            _ => Err(AttributeError::InvalidType)
        }
    }
}

/// converts the duration to and from String using the `S` attribute
#[cfg(any(feature = "time", feature = "full"))]
impl Attribute for Duration {
    fn attribute(&self) -> AttributeValue {
        let value: String = DurationString::from(self.clone()).into();
        AttributeValue::S(value)
    }

    fn value(value: AttributeValue) -> Result<Self, AttributeError> {
        match value {
            AttributeValue::S(duration) => Ok(DurationString::from_string(duration)?.into()),
            _ => Err(AttributeError::InvalidType)
        }
    }
}

/// An `rfc3339` formatted version of `DateTime<Utc>`, represented by the `S` AttributeValue type
#[cfg(any(feature = "time", feature = "full"))]
impl Attribute for DateTime<Utc> {
    fn attribute(&self) -> AttributeValue {
        let time = self.to_rfc3339();
        AttributeValue::S(time)
    }

    fn value(value: AttributeValue) -> Result<Self, AttributeError> {
        match value {
            AttributeValue::S(string) => {
                match DateTime::parse_from_rfc3339(&string).map(|dt| dt.with_timezone(&Utc)) {
                    Ok(date_time) => Ok(date_time),
                    Err(_) => Err(AttributeError::InvalidFormat)?,
                }
            },
            _ => Err(AttributeError::InvalidType)
        }
    }
}

/// An `rfc3339` formatted version of `DateTime<FixedOffset>`, represented by the `S` AttributeValue type
#[cfg(any(feature = "time", feature = "full"))]
impl Attribute for DateTime<FixedOffset> {
    fn attribute(&self) -> AttributeValue {
        let time = self.to_rfc3339();
        AttributeValue::S(time)
    }

    fn value(value: AttributeValue) -> Result<Self, AttributeError> {
        match value {
            AttributeValue::S(string) => {
                match DateTime::parse_from_rfc3339(&string).map(|dt| dt) {
                    Ok(date_time) => Ok(date_time),
                    Err(_) => Err(AttributeError::InvalidFormat)?,
                }
            },
            _ => Err(AttributeError::InvalidType)
        }
    }
}

/// An `rfc3339` formatted version of `SystemTime`, represented by the `S` AttributeValue type
#[cfg(any(feature = "time", feature = "full"))]
impl Attribute for SystemTime {
    fn attribute(&self) -> AttributeValue {
        let date = Into::<DateTime<Utc>>::into(self.clone());
        date.attribute()
    }

    fn value(value: AttributeValue) -> Result<Self, AttributeError> {
        let date = DateTime::<Utc>::value(value)?;
        Ok(date.into())
    }
}

/// An `rfc3339` formatted version of `DateTime<Local>`, represented by the `S` AttributeValue type
#[cfg(any(feature = "time", feature = "full"))]
impl Attribute for DateTime<Local> {
    fn attribute(&self) -> AttributeValue {
        let time = self.to_rfc3339();
        AttributeValue::S(time)
    }

    fn value(value: AttributeValue) -> Result<Self, AttributeError> {
        match value {
            AttributeValue::S(string) => {
                match DateTime::parse_from_rfc3339(&string).map(|dt| dt.with_timezone(&Local)) {
                    Ok(date_time) => Ok(date_time),
                    Err(_) => Err(AttributeError::InvalidFormat)?,
                }
            },
            _ => Err(AttributeError::InvalidType)
        }
    }
}

/// An `rfc3339` formatted version of `NaiveDate`, represented by the `S` AttributeValue type
#[cfg(any(feature = "time", feature = "full"))]
impl Attribute for chrono::naive::NaiveDate {
    fn attribute(&self) -> AttributeValue {
        AttributeValue::S(self.to_string())
    }

    fn value(value: AttributeValue) -> Result<Self, AttributeError> {
        match value {
            AttributeValue::S(s) => Ok(Self::from_str(&s)?),
            _ => Err(AttributeError::InvalidType)
        }
    }
}