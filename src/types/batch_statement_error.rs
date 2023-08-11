use super::*;


/// <p> An error associated with a statement in a PartiQL batch that was run. </p>
#[derive(Debug, Clone, PartialEq, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct BatchStatamentError {
    /// The error code associated with the failed PartiQL batch statement.
    pub code: Option<BatchStatamentErrorCode>,
    /// The item which caused the condition check to fail. This will be set if ReturnValuesOnConditionCheckFailure is specified as ALL_OLD.
    pub item: Option<HashMap<String, AttributeValue>>,
    /// The error message associated with the PartiQL batch response.
    pub message: Option<String>
}