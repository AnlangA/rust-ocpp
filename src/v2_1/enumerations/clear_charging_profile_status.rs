use serde::{Deserialize, Serialize};

/// Indicates if the Charging Station was able to execute the request.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[derive(Default)]
pub enum ClearChargingProfileStatusEnumType {
    #[default]
    Accepted,
    Unknown,
}

