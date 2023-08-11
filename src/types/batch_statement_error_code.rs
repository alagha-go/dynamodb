use super::*;


/// <p> possible error code  values from batch statement operation </p>
#[derive(Debug, Clone, PartialEq, Deserialize)]
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