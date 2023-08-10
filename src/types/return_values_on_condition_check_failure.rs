use super::*;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ReturnValuesOnConditionCheckFailure {
    #[default]
    None,
    AllOld,
}