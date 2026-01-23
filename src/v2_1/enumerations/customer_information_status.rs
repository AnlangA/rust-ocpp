use serde::{Deserialize, Serialize};

/// Indicates whether the request was accepted.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[derive(Default)]
pub enum CustomerInformationStatusEnumType {
    #[serde(rename = "Accepted")]
    #[default]
    Accepted,
    #[serde(rename = "Rejected")]
    Rejected,
    #[serde(rename = "Invalid")]
    Invalid,
}

