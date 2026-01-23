use serde::{Deserialize, Serialize};

/// Accepted if the Charging Station has executed the request, otherwise rejected.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[derive(Default)]
pub enum ClearCacheStatusEnumType {
    #[serde(rename = "Accepted")]
    #[default]
    Accepted,
    #[serde(rename = "Rejected")]
    Rejected,
}

