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