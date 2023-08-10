use super::*;


/// <p> A PartiQL batch statement request. </p>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase", default)]
pub struct BatchStatementRequest {
    /// <p> The read consistency of the PartiQL batch request. </p>
    pub consistent_read: bool,
    /// <p> The parameters associated with a PartiQL statement in the batch request. </p>
    pub parameters: Option<Vec<AttributeValue>>,
    /// <p>An optional parameter that returns the item attributes for a PartiQL batch request operation that failed a condition check.</p>
    /// <p>There is no additional cost associated with requesting a return value aside from the small network and processing overhead of receiving a larger response. No read capacity units are consumed.</p>
    pub return_values_on_condition_check_failure: ReturnValuesOnConditionCheckFailure,
    /// <p> A valid PartiQL statement. </p>
    pub statement: Option<String>,
}