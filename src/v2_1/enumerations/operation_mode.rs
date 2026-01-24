use serde::{Deserialize, Serialize};

/// Charging operation mode to use during this time interval. When absent defaults to `ChargingOnly`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum OperationModeEnumType {
    Idle,
    ChargingOnly,
    CentralSetpoint,
    ExternalSetpoint,
    ExternalLimits,
    CentralFrequency,
    LocalFrequency,
    LocalLoadBalancing,
}
