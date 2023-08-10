use super::*;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ReturnConsumedCapacity {
    #[default]
    None,
    Indexes,
    Total
}