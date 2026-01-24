use serde::{Deserialize, Serialize};

/// Accepted if the Charging Station has executed the request, otherwise rejected.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[derive(Default)]
pub enum ClearCacheStatusEnumType {
    #[default]
    Accepted,
    Rejected,
}
