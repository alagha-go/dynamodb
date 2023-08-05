use super::*;

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub enum AttributeValue {
    #[default]
    #[serde(with="null", rename = "NULL")]
    None,
    BOOL(bool),
    S(String),
    N(String),
    B(Binary),
    SS(Vec<String>),
    NS(Vec<String>),
    BS(Vec<Binary>),
    M(HashMap<String, AttributeValue>),
    L(Vec<AttributeValue>)
}


impl<A: Attribute, B: Attribute> Attribute for (A, B) {
    fn attribute(&self) -> AttributeValue {
        AttributeValue::L(vec![self.0.attribute(), self.1.attribute()])
    }

    fn value(attribute: AttributeValue) -> Result<Self> {
        match attribute {
            AttributeValue::L(mut list) => {
                Ok((Attribute::value(list.remove(0))?, Attribute::value(list.remove(0))?))
            },
            _ => Err(StdError::from("invalid tuple expected a array to convert it to tuple"))
        }
    }
}
