use serde::{Deserialize, Serialize};

/// Battery in/out
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum BatterySwapEventEnumType {
    BatteryIn,
    BatteryOut,
    BatteryOutTimeout,
}
