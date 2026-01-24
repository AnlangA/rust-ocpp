use serde::{Deserialize, Serialize};

/// Current charging state, is required when state has changed.
/// Omitted when there is no communication between EVSE and EV, because no cable is plugged in.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum ChargingStateEnumType {
    EVConnected,
    Charging,
    SuspendedEV,
    SuspendedEVSE,
    Idle,
}
