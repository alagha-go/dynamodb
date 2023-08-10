use super::*;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct BatchExecuteStatementResponse {
    consumed_capacity: Vec<ConsumedCapacity>,
    responses: Vec<BatchStatementResponse>
}