use super::*;

///<p> An optional parameter that returns the item attributes for a PartiQL batch request operation that failed a condition check. </p>
///
/// <p> There is no additional cost associated with requesting a return value aside from the small network and processing overhead of receiving a larger response. No read capacity units are consumed. </p>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ReturnValuesOnConditionCheckFailure {
    #[default]
    /// no value will be returned
    None,
    /// old values will be returned
    AllOld,
}