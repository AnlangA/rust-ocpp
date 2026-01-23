use serde::{Deserialize, Serialize};

/// Indicates if the Charging Station was able to execute the request.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[derive(Default)]
pub enum ClearChargingProfileStatusEnumType {
    #[serde(rename = "Accepted")]
    #[default]
    Accepted,
    #[serde(rename = "Unknown")]
    Unknown,
}

