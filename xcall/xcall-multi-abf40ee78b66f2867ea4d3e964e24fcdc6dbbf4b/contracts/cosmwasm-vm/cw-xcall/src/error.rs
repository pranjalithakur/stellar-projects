use super::*;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),
    #[error("Unauthorized")]
    Unauthorized {},
    #[error("ERR_REPLY_ERROR|{code:?}|{msg:?}")]
    ReplyError { code: u64, msg: String },
    #[error("Admin Already Exist")]
    AdminAlreadyExist,
    #[error("OwnerAlreadyExist")]
    AdminNotExist,
    #[error("RollbackNotPossible")]
    RollbackNotPossible,
    #[error("MaxDataSizeExceeded")]
    MaxDataSizeExceeded,
    #[error("MaxRollbackSizeExceeded")]
    MaxRollbackSizeExceeded,
    #[error("NotExistRequestId {id}")]
    NotExistRequestId { id: u128 },
    #[error("InvalidRequestId {id}")]
    InvalidRequestId { id: u128 },
    #[error("RollbackNotEnabled")]
    RollbackNotEnabled,
    #[error("InvalidSequenceId {id}")]
    InvalidSequenceId { id: u128 },
    #[error("DecodeFailed {error}")]
    DecodeFailed { error: String },
    #[error("OnlyAdmin")]
    OnlyAdmin,
    #[error("AdminAddressCannotBeNull")]
    AdminAddressCannotBeNull {},
    #[error("InvalidAddress {address}")]
    InvalidAddress { address: String },
    #[error("InsufficientFunds")]
    InsufficientFunds,
    #[error("ProtocolsMismatch")]
    ProtocolsMismatch,
    #[error("DataMismatch")]
    DataMismatch,
    #[error("CallAlreadyInProgress")]
    CallAlreadyInProgress,
    #[error("MessageTypeNotAllowed")]
    MessageTypeNotAllowed,
    #[error("InvalidReplyReceived")]
    InvalidReplyReceived,
    #[error("CallRequest Not Found For {sn}")]
    CallRequestNotFound { sn: u128 },
}
