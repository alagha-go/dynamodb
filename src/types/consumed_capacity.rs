use super::*;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct ConsumedCapacity {
    pub capacity_units: Option<f64>,
    pub read_capacity_units: Option<f64>,
    pub write_capacity_units: Option<f64>,
    pub global_secondary_index: Option<HashMap<String, Capacity>>,
    pub local_secondary_index: Option<HashMap<String, Capacity>>,
    pub table: Option<Capacity>
}