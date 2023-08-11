use super::*;


/// <p> Determines the level of detail about either provisioned or on-demand throughput consumption that is returned in the response. </p>
#[derive(Debug, Clone, PartialEq, Serialize, Default)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ReturnConsumedCapacity {
    #[default]
    /// No [`ConsumedCapacity`] details are included in the response.
    None,
    /// The response includes the aggregate `ConsumedCapacity` for the operation, together with `ConsumedCapacity` for each table and secondary index that was accessed.
    ///
    ///`Note` that some operations, such as `GetItem` and `BatchGetItem`, do not access any indexes at all. In these cases, specifying `INDEXES` will only return `ConsumedCapacity` information for tables.
    Indexes,
    /// The response includes only the aggregate `onsumedCapacity` for the operation.
    Total
}