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


impl Attribute for AttributeValue {
    fn attribute(&self) -> AttributeValue {
        self.clone()
    }

    fn value(attribute: AttributeValue) -> AttributeResult<Self> {
        Ok(attribute)
    }
}