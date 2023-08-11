use super::*;


/// <p> A PartiQL batch statement response. </p>
#[derive(Debug, Clone, PartialEq, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct BatchExecuteStatementResponse {
    /// The capacity units consumed by the entire operation. The values of the list are ordered according to the ordering of the statements.
    consumed_capacity: Vec<ConsumedCapacity>,
    /// The response to each PartiQL statement in the batch. The values of the list are ordered according to the ordering of the request statements.
    responses: Vec<BatchStatementResponse>
}