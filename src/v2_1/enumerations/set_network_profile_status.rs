use serde::{Deserialize, Serialize};

/// Status of setting a network profile.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub enum SetNetworkProfileStatusEnumType {
    /// Request has been accepted and the network profile has been set.
    #[default]
    Accepted,

    /// Request has been rejected.
    Rejected,

    /// Request has been accepted but failed to be applied.
    Failed,
}
