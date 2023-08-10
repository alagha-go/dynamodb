use super::*;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct BatchStatementResponse {
    pub error: Option<BatchStatamentError>,
    pub item: Option<HashMap<String, AttributeValue>>,
    pub table_name: Option<String>
}