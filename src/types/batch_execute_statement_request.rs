use super::*;


/// <p> A PartiQL batch statement request. </p>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct BatchExecuteStatementRequest {
    /// <p>The list of PartiQL statements representing the batch to run.</p>
    /// `Note`: The entire batch must consist of either read statements or write statements, you cannot mix both in one batch.
    pub statements: Vec<BatchStatementRequest>,
    /// <p>Determines the level of detail about either provisioned or on-demand throughput consumption that is returned in the response:</p>
    /// <ul>
    /// <li> <p> <code>INDEXES</code> - The response includes the aggregate <code>ConsumedCapacity</code> for the operation, together with <code>ConsumedCapacity</code> for each table and secondary index that was accessed.</p> <p>Note that some operations, such as <code>GetItem</code> and <code>BatchGetItem</code>, do not access any indexes at all. In these cases, specifying <code>INDEXES</code> will only return <code>ConsumedCapacity</code> information for table(s).</p> </li>
    /// <li> <p> <code>TOTAL</code> - The response includes only the aggregate <code>ConsumedCapacity</code> for the operation.</p> </li>
    /// <li> <p> <code>NONE</code> - No <code>ConsumedCapacity</code> details are included in the response.</p> </li>
    /// </ul>
    pub return_consumed_capacity: ReturnConsumedCapacity,
}