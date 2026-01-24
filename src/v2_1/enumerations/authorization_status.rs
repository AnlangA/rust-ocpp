use serde::{Deserialize, Serialize};

/// Current status of the ID Token.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum AuthorizationStatusEnumType {
    Accepted,
    Blocked,
    ConcurrentTx,
    Expired,
    Invalid,
    NoCredit,
    NotAllowedTypeEVSE,
    NotAtThisLocation,
    NotAtThisTime,
    Unknown,
}
