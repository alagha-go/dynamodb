use super::*;


/// <p> Represents the amount of provisioned throughput capacity consumed on a table or an index. </p>
#[derive(Debug, Clone, PartialEq, Deserialize, Default)]
#[serde(rename_all = "PascalCase", default)]
pub struct Capacity {
    /// The total number of capacity units consumed on a table or an index.
    pub capacity_units: Option<f64>,
    /// The total number of read capacity units consumed on a table or an index.
    pub read_capacity_units: Option<f64>,
    /// The total number of write capacity units consumed on a table or an index.
    pub write_capacity_units: Option<f64>,
}
