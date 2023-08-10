use super::*;


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct Capacity {
    pub capacity_units: Option<f64>,
    pub read_capacity_units: Option<f64>,
    pub write_capacity_units: Option<f64>,
}
