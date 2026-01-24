use serde::{Deserialize, Serialize};

/// Indicates the kind of schedule.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum ChargingProfileKindEnumType {
    Absolute,
    Recurring,
    Relative,
    Dynamic,
}
