use serde::{Deserialize, Serialize};

/// Result of operation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[derive(Default)]
pub enum DERControlStatusEnumType {
    #[serde(rename = "Accepted")]
    #[default]
    Accepted,
    #[serde(rename = "Rejected")]
    Rejected,
    #[serde(rename = "NotSupported")]
    NotSupported,
    #[serde(rename = "NotFound")]
    NotFound,
}

