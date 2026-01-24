use serde::{Deserialize, Serialize};

/// Returns whether the Charging Station has been able to remove the message.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[derive(Default)]
pub enum ClearMessageStatusEnumType {
    Accepted,
    #[default]
    Unknown,
    Rejected,
}
