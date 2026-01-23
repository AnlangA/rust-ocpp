use serde::{Deserialize, Serialize};

/// Returns whether the Charging Station has been able to remove the message.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[derive(Default)]
pub enum ClearMessageStatusEnumType {
    #[serde(rename = "Accepted")]
    Accepted,
    #[serde(rename = "Unknown")]
    #[default]
    Unknown,
    #[serde(rename = "Rejected")]
    Rejected,
}

