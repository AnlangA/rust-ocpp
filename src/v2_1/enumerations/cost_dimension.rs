use serde::{Deserialize, Serialize};

/// Type of cost dimension: energy, power, time, etc.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[derive(Default)]
pub enum CostDimensionEnumType {
    #[default]
    Energy,
    MaxCurrent,
    MinCurrent,
    MaxPower,
    MinPower,
    IdleTime,
    ChargingTime,
}

