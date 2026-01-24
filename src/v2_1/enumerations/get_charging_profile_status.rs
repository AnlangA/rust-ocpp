use serde::{Deserialize, Serialize};

/// This indicates whether the Charging Station is able to process this request and will send `ReportChargingProfilesRequest` messages.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[derive(Default)]
pub enum GetChargingProfileStatusEnumType {
    #[default]
    Accepted,
    NoProfiles,
}

