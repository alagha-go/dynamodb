use super::*;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct BatchExecuteStatementRequest {
    pub return_consumed_capacity: ReturnConsumedCapacity,
    pub statements: Vec<BatchExecuteStatementRequest>,
}