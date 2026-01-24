use serde::{Deserialize, Serialize};

/// Result of operation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[derive(Default)]
pub enum DERControlStatusEnumType {
    #[default]
    Accepted,
    Rejected,
    NotSupported,
    NotFound,
}
