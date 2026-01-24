use serde::{Deserialize, Serialize};

/// Type of EVSE (AC, DC) this tariff applies to.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum EvseKindEnumType {
    AC,
    DC,
}
