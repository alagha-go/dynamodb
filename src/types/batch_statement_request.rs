use super::*;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct BatchStatementRequest {
    pub consistent_read: bool,
    pub parameters: Option<Vec<AttributeValue>>,
    pub return_values_on_condition_check_failure: ReturnValuesOnConditionCheckFailure,
    pub statement: Option<String>,
}