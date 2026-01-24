use serde::{Deserialize, Serialize};

/// This indicates whether the Charging Station is able to perform the availability change.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum ChangeAvailabilityStatusEnumType {
    Accepted,
    Rejected,
    Scheduled,
}
