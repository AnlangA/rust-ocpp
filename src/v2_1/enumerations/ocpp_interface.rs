use serde::{Deserialize, Serialize};

/// Applicable Network Interface. Charging Station is allowed to use a different network interface
/// to connect if the given one does not work.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum OCPPInterfaceEnumType {
    Wired0,
    Wired1,
    Wired2,
    Wired3,
    Wireless0,
    Wireless1,
    Wireless2,
    Wireless3,
    Any,
}
