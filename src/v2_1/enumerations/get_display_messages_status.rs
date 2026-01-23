use serde::{Deserialize, Serialize};

/// Indicates if the Charging Station has Display Messages that match the request criteria in the GetDisplayMessagesRequest.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[derive(Default)]
pub enum GetDisplayMessagesStatusEnumType {
    #[serde(rename = "Accepted")]
    #[default]
    Accepted,
    #[serde(rename = "Unknown")]
    Unknown,
}

