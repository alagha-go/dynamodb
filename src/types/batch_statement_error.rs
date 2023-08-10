use super::*;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct BatchStatamentError {
    pub code: Option<BatchStatamentErrorCode>,
    pub item: Option<HashMap<String, AttributeValue>>,
    pub message: Option<String>
}