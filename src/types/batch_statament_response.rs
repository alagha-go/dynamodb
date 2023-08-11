use super::*;


/// <p> A PartiQL batch statement response </p>
#[derive(Debug, Clone, PartialEq, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct BatchStatementResponse {
    /// The error associated with a failed PartiQL batch statement.
    pub error: Option<BatchStatamentError>,
    /// A DynamoDB item associated with a BatchStatementResponse
    pub item: Option<HashMap<String, AttributeValue>>,
    /// The table name associated with a failed PartiQL batch statement.
    pub table_name: Option<String>
}