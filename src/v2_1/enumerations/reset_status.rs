use serde::{Deserialize, Serialize};

/// This indicates whether the Charging Station is able to perform the reset.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum ResetStatusEnumType {
    /// Reset request has been accepted and will be performed.
    Accepted,

    /// Reset request has been rejected.
    Rejected,

    /// Reset request has been scheduled for later execution.
    Scheduled,
}
