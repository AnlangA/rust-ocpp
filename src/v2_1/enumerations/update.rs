use serde::{Deserialize, Serialize};

/// Type of update for a local authorization list.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub enum UpdateEnumType {
    /// Only send the changes in the local authorization list.
    #[default]
    Differential,

    /// Send the full local authorization list.
    Full,
}
