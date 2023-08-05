use super::*;

pub trait Attribute: Sized {
    fn attribute(&self) -> AttributeValue;
    fn value(value: AttributeValue) -> Result<Self>;
    fn option_value(option: Option<AttributeValue>) -> Result<Self> {
        match option {
            None => Err(StdError::from("expected AttributeValue but found None")),
            Some(attribute) => Attribute::value(attribute)
        }
    }
    fn vec_attribute(values: &[Self]) -> AttributeValue {
        AttributeValue::L(values.into_iter().map(|value| {value.attribute()}).collect::<Vec<AttributeValue>>())
    }
    
}

fn value<T: Attribute>(attribute: AttributeValue) -> Result<T> {
    Attribute::value(attribute)
}

impl Attribute for u8 {
    fn attribute(&self) -> AttributeValue {
        AttributeValue::N(self.to_string())
    }

    fn value(attribute: AttributeValue) -> Result<Self> {
        match attribute {
            AttributeValue::N(number) => Ok(number.parse()?),
            _ => Err(StdError::from("not a valid number"))
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

    fn value(attribute: AttributeValue) -> Result<Self> {
        match attribute {
            AttributeValue::N(number) => Ok(number.parse()?),
            _ => Err(StdError::from("not a valid number"))
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

    fn value(attribute: AttributeValue) -> Result<Self> {
        match attribute {
            AttributeValue::N(number) => Ok(number.parse()?),
            _ => Err(StdError::from("not a valid number"))
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

    fn value(attribute: AttributeValue) -> Result<Self> {
        match attribute {
            AttributeValue::N(number) => Ok(number.parse()?),
            _ => Err(StdError::from("not a valid number"))
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

    fn value(attribute: AttributeValue) -> Result<Self> {
        match attribute {
            AttributeValue::N(number) => Ok(number.parse()?),
            _ => Err(StdError::from("not a valid number"))
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

    fn value(attribute: AttributeValue) -> Result<Self> {
        match attribute {
            AttributeValue::N(number) => Ok(number.parse()?),
            _ => Err(StdError::from("not a valid number"))
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

    fn value(attribute: AttributeValue) -> Result<Self> {
        match attribute {
            AttributeValue::N(number) => Ok(number.parse()?),
            _ => Err(StdError::from("not a valid number"))
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

    fn value(attribute: AttributeValue) -> Result<Self> {
        match attribute {
            AttributeValue::N(number) => Ok(number.parse()?),
            _ => Err(StdError::from("not a valid number"))
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

    fn value(attribute: AttributeValue) -> Result<Self> {
        match attribute {
            AttributeValue::N(number) => Ok(number.parse()?),
            _ => Err(StdError::from("not a valid number"))
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

    fn value(attribute: AttributeValue) -> Result<Self> {
        match attribute {
            AttributeValue::N(number) => Ok(number.parse()?),
            _ => Err(StdError::from("not a valid number"))
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

    fn value(attribute: AttributeValue) -> Result<Self> {
        match attribute {
            AttributeValue::N(number) => Ok(number.parse()?),
            _ => Err(StdError::from("not a valid number"))
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

    fn value(attribute: AttributeValue) -> Result<Self> {
        match attribute {
            AttributeValue::N(number) => Ok(number.parse()?),
            _ => Err(StdError::from("not a valid number"))
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

    fn value(attribute: AttributeValue) -> Result<Self> {
        match attribute {
            AttributeValue::N(number) => Ok(number.parse()?),
            _ => Err(StdError::from("not a valid number"))
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

    fn value(attribute: AttributeValue) -> Result<Self> {
        match attribute {
            AttributeValue::N(number) => Ok(number.parse()?),
            _ => Err(StdError::from("not a valid number"))
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

    fn value(attribute: AttributeValue) -> Result<Self> {
        match attribute {
            AttributeValue::BOOL(b) => Ok(b),
            _ => Err(StdError::from("not a valid bool"))
        }
    }
}

impl Attribute for String {
    fn attribute(&self) -> AttributeValue {
        AttributeValue::S(self.to_owned())
    }

    fn value(attribute: AttributeValue) -> Result<Self> {
        match attribute {
            AttributeValue::S(string) => Ok(string),
            AttributeValue::N(string) => Ok(string),
            _ => Err(StdError::from("not a valid string"))
        }
    }

    fn vec_attribute(values: &[Self]) -> AttributeValue {
        AttributeValue::SS(values.into_iter().map(|value| {value.to_string()}).collect())
    }

}

impl<V: Attribute> Attribute for Option<V> {
    fn attribute(&self) -> AttributeValue {
        match self {
            None => AttributeValue::None,
            Some(value) => value.attribute()
        }
    }

    fn value(attribute: AttributeValue) -> Result<Self> {
        match attribute {
            AttributeValue::None => Ok(None),
            _ => Ok(Some(Attribute::value(attribute)?))
        }
    }
}

impl<V: Attribute + Clone> Attribute for Vec<V> {
    fn attribute(&self) -> AttributeValue {
        V::vec_attribute(self)
    }

    fn value(attribute: AttributeValue) -> Result<Self> {
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
            _ => Err(StdError::from("not a valid array"))
        }
    }
}

impl Attribute for Vec<Binary> {
    fn attribute(&self) -> AttributeValue {
        AttributeValue::BS(self.clone())
    }

    fn value(attribute: AttributeValue) -> Result<Self> {
        match attribute {
            AttributeValue::BS(binaries) => Ok(binaries),
            _ => Err(StdError::from("not a valid array of binaries"))
        }
    }
}

impl<K: From<String> + Into<String> + Clone + Eq + std::hash::Hash, V: Attribute + Clone> Attribute for HashMap<K, V> {
    fn attribute(&self) -> AttributeValue {
        let mut map = HashMap::new();
        let _ = self.into_iter().map(|(key, value)|{map.insert(key.clone().into(), value.attribute())});
        AttributeValue::M(map)
    }

    fn value(attribute: AttributeValue) -> Result<Self> {
        match attribute {
            AttributeValue::M(object) => {
                let mut map = HashMap::new();
                for (key, value) in object {
                    map.insert(key.into(), V::value(value)?);
                }
                Ok(map)
            },
            _ => Ok(HashMap::new())
        }
    }
}

impl Attribute for AttributeValue {
    fn attribute(&self) -> AttributeValue {
        self.clone()
    }

    fn value(attribute: AttributeValue) -> Result<Self> {
        Ok(attribute)
    }
}

impl<A: Attribute> Attribute for (A,) {
    fn attribute(&self) -> AttributeValue {
        self.0.attribute()
    }

    fn value(attribute: AttributeValue) -> Result<Self> {
        Ok((A::value(attribute)?,))
    }
}

impl<A: Attribute, B: Attribute, C: Attribute> Attribute for (A, B, C) {
    fn attribute(&self) -> AttributeValue {
        AttributeValue::L(vec![self.0.attribute(), self.1.attribute(), self.2.attribute()])
    }

    fn value(attribute: AttributeValue) -> Result<Self> {
        match attribute {
            AttributeValue::L(mut list) => {
                Ok((Attribute::value(list.remove(0))?, Attribute::value(list.remove(0))?, Attribute::value(list.remove(0))?))
            },
            _ => Err(StdError::from("invalid tuple expected an array to convert it to tuple"))
        }
    }
}

impl<A: Attribute, B: Attribute, C: Attribute, D: Attribute> Attribute for (A, B, C, D) {
    fn attribute(&self) -> AttributeValue {
        AttributeValue::L(vec![self.0.attribute(), self.1.attribute(), self.2.attribute(), self.3.attribute()])
    }

    fn value(attribute: AttributeValue) -> Result<Self> {
        match attribute {
            AttributeValue::L(mut list) => {
                Ok((Attribute::value(list.remove(0))?, Attribute::value(list.remove(0))?, Attribute::value(list.remove(0))?, Attribute::value(list.remove(0))?))
            },
            _ => Err(StdError::from("invalid tuple expected an array to convert it to tuple"))
        }
    }
}

impl<A: Attribute, B: Attribute, C: Attribute, D: Attribute, E: Attribute> Attribute for (A, B, C, D, E) {
    fn attribute(&self) -> AttributeValue {
        AttributeValue::L(vec![self.0.attribute(), self.1.attribute(), self.2.attribute(), self.3.attribute(), self.4.attribute()])
    }

    fn value(attribute: AttributeValue) -> Result<Self> {
        match attribute {
            AttributeValue::L(mut list) => {
                Ok((Attribute::value(list.remove(0))?, Attribute::value(list.remove(0))?, Attribute::value(list.remove(0))?, Attribute::value(list.remove(0))?, Attribute::value(list.remove(0))?))
            },
            _ => Err(StdError::from("invalid tuple expected an array to convert it to tuple"))
        }
    }
}

impl<A: Attribute, B: Attribute, C: Attribute, D: Attribute, E: Attribute, F: Attribute> Attribute for (A, B, C, D, E, F) {
    fn attribute(&self) -> AttributeValue {
        AttributeValue::L(vec![self.0.attribute(), self.1.attribute(), self.2.attribute(), self.3.attribute(), self.4.attribute(), self.5.attribute()])
    }

    fn value(attribute: AttributeValue) -> Result<Self> {
        match attribute {
            AttributeValue::L(mut list) => {
                Ok((value(list.remove(0))?, value(list.remove(0))?, value(list.remove(0))?, value(list.remove(0))?, value(list.remove(0))?, value(list.remove(0))?))
            },
            _ => Err(StdError::from("invalid tuple expected an array to convert it to tuple"))
        }
    }
}

impl<A: Attribute, B: Attribute, C: Attribute, D: Attribute, E: Attribute, F: Attribute, G: Attribute> Attribute for (A, B, C, D, E, F, G) {
    fn attribute(&self) -> AttributeValue {
        AttributeValue::L(vec![self.0.attribute(), self.1.attribute(), self.2.attribute(), self.3.attribute(), self.4.attribute(), self.5.attribute(), self.6.attribute()])
    }

    fn value(attribute: AttributeValue) -> Result<Self> {
        match attribute {
            AttributeValue::L(mut list) => {
                Ok((value(list.remove(0))?, value(list.remove(0))?, value(list.remove(0))?, value(list.remove(0))?, value(list.remove(0))?, value(list.remove(0))?, value(list.remove(0))?))
            },
            _ => Err(StdError::from("invalid tuple expected an array to convert it to tuple"))
        }
    }
}

impl<A: Attribute, B: Attribute, C: Attribute, D: Attribute, E: Attribute, F: Attribute, G: Attribute, H: Attribute> Attribute for (A, B, C, D, E, F, G, H) {
    fn attribute(&self) -> AttributeValue {
        AttributeValue::L(vec![self.0.attribute(), self.1.attribute(), self.2.attribute(), self.3.attribute(), self.4.attribute(), self.5.attribute(), self.6.attribute(), self.7.attribute()])
    }

    fn value(attribute: AttributeValue) -> Result<Self> {
        match attribute {
            AttributeValue::L(mut list) => {
                Ok((value(list.remove(0))?, value(list.remove(0))?, value(list.remove(0))?, value(list.remove(0))?, value(list.remove(0))?, value(list.remove(0))?, value(list.remove(0))?, value(list.remove(0))?))
            },
            _ => Err(StdError::from("invalid tuple expected an array to convert it to tuple"))
        }
    }
}

impl<A: Attribute, B: Attribute, C: Attribute, D: Attribute, E: Attribute, F: Attribute, G: Attribute, H: Attribute, I: Attribute> Attribute for (A, B, C, D, E, F, G, H, I) {
    fn attribute(&self) -> AttributeValue {
        AttributeValue::L(vec![self.0.attribute(), self.1.attribute(), self.2.attribute(), self.3.attribute(), self.4.attribute(), self.5.attribute(), self.6.attribute(), self.7.attribute(), self.8.attribute()])
    }

    fn value(attribute: AttributeValue) -> Result<Self> {
        match attribute {
            AttributeValue::L(mut list) => {
                Ok((value(list.remove(0))?, value(list.remove(0))?, value(list.remove(0))?, value(list.remove(0))?, value(list.remove(0))?, value(list.remove(0))?, value(list.remove(0))?, value(list.remove(0))?, value(list.remove(0))?))
            },
            _ => Err(StdError::from("invalid tuple expected an array to convert it to tuple"))
        }
    }
}

impl<A: Attribute, B: Attribute, C: Attribute, D: Attribute, E: Attribute, F: Attribute, G: Attribute, H: Attribute, I: Attribute, J: Attribute> Attribute for (A, B, C, D, E, F, G, H, I, J) {
    fn attribute(&self) -> AttributeValue {
        AttributeValue::L(vec![self.0.attribute(), self.1.attribute(), self.2.attribute(), self.3.attribute(), self.4.attribute(), self.5.attribute(), self.6.attribute(), self.7.attribute(), self.8.attribute(), self.9.attribute()])
    }

    fn value(attribute: AttributeValue) -> Result<Self> {
        match attribute {
            AttributeValue::L(mut list) => {
                Ok((value(list.remove(0))?, value(list.remove(0))?, value(list.remove(0))?, value(list.remove(0))?, value(list.remove(0))?, value(list.remove(0))?, value(list.remove(0))?, value(list.remove(0))?, value(list.remove(0))?, value(list.remove(0))?))
            },
            _ => Err(StdError::from("invalid tuple expected an array to convert it to tuple"))
        }
    }
}

impl <T: Attribute + Default, const SIZE: usize> Attribute for [T; SIZE] {
    fn attribute(&self) -> AttributeValue {
        Attribute::vec_attribute(self)
    }

    fn value(attribute: AttributeValue) -> Result<Self> {
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
                    Err(_) => Err(StdError::from(format!("unexpected error occured. could not convert from a Vec<T> to [T; {}]", SIZE)))
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
                    Err(_) => Err(StdError::from(format!("unexpected error occured. could not convert from a Vec<T> to [T; {}]", SIZE)))
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
                    Err(_) => Err(StdError::from(format!("unexpected error occured. could not convert from a Vec<T> to [T; {}]", SIZE)))
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
                    Err(_) => Err(StdError::from(format!("unexpected error occured. could not convert from a Vec<T> to [T; {}]", SIZE)))
                }
            }
            _ => Err(StdError::from("expected an array of items"))
        }
    }
}

impl<T: Attribute + Clone> Attribute for VecDeque<T> {
    fn attribute(&self) -> AttributeValue {
        let values = Into::<Vec<T>>::into(self.clone());
        T::vec_attribute(&values)
    }

    fn value(value: AttributeValue) -> Result<Self> {
        let array: Vec<T> = Attribute::value(value)?;
        Ok(array.into())
    }
}

impl <T: Attribute> Attribute for Box<T> {
    fn attribute(&self) -> AttributeValue {
        let value = &**self;
        Attribute::attribute(value)
    }

    fn value(value: AttributeValue) -> Result<Self> {
        let value = Attribute::value(value)?;
        Ok(Box::new(value))
    }
}

impl <T: Attribute> Attribute for Rc<T> {
    fn attribute(&self) -> AttributeValue {
        let value = &**self;
        Attribute::attribute(value)
    }

    fn value(value: AttributeValue) -> Result<Self> {
        let value = T::value(value)?;
        Ok(Rc::new(value))
    }
}
