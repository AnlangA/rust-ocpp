use serde::{Deserialize, Serialize};

/// Status of operation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[derive(Default)]
pub enum GenericStatusEnumType {
    #[default]
    Accepted,
    Rejected,
}
