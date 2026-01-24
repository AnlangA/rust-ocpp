use serde::{Deserialize, Serialize};

/// Defines the purpose of the schedule transferred by this profile
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum ChargingProfilePurposeEnumType {
    ChargingStationExternalConstraints,
    ChargingStationMaxProfile,
    TxDefaultProfile,
    TxProfile,
    PriorityCharging,
    LocalGeneration,
}
