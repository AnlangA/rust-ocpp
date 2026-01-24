use serde::{Deserialize, Serialize};

/// The unit of measure in which limits and setpoints are expressed.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum ChargingRateUnitEnumType {
    W,
    A,
}
