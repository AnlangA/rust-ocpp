use serde::{Deserialize, Serialize};

/// Indicates whether the request was accepted.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[derive(Default)]
pub enum CustomerInformationStatusEnumType {
    #[default]
    Accepted,
    Rejected,
    Invalid,
}
