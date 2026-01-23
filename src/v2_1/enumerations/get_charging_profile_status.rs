use serde::{Deserialize, Serialize};

/// This indicates whether the Charging Station is able to process this request and will send ReportChargingProfilesRequest messages.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[derive(Default)]
pub enum GetChargingProfileStatusEnumType {
    #[serde(rename = "Accepted")]
    #[default]
    Accepted,
    #[serde(rename = "NoProfiles")]
    NoProfiles,
}

