use super::*;


/// <p> The capacity units consumed by the entire operation.</p>
/// <p> The data returned includes the total provisioned throughput consumed, along with statistics for the table and any indexes involved in the operation. </p>
#[derive(Debug, Clone, PartialEq, Deserialize, Default)]
#[serde(rename_all = "PascalCase", default)]
pub struct ConsumedCapacity {
    /// The total number of capacity units consumed by the operation.
    pub capacity_units: Option<f64>,
    /// The total number of read capacity units consumed by the operation.
    pub read_capacity_units: Option<f64>,
    /// The total number of write capacity units consumed by the operation.
    pub write_capacity_units: Option<f64>,
    /// The amount of throughput consumed on each global index affected by the operation.
    pub global_secondary_index: Option<HashMap<String, Capacity>>,
    /// The amount of throughput consumed on each local index affected by the operation.
    pub local_secondary_index: Option<HashMap<String, Capacity>>,
    /// The amount of throughput consumed on the table affected by the operation.
    pub table: Option<Capacity>
}