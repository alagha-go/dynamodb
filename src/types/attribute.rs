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