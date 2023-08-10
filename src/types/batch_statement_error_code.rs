use super::*;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum BatchStatamentErrorCode {
    ConditionalCheckFailed,
    ItemCollectionSizeLimitExceeded,
    RequestLimitExceeded,
    ValidationError,
    ProvisionedThroughputExceeded,
    TransactionConflict,
    ThrottlingError,
    InternalServerError,
    ResourceNotFound,
    AccessDenied,
    DuplicateItem
}