use serde::{Deserialize, Serialize};

/// Indicates if the Charging Station has Display Messages that match the request criteria in the GetDisplayMessagesRequest.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[derive(Default)]
pub enum GetDisplayMessagesStatusEnumType {
    #[default]
    Accepted,
    Unknown,
}

