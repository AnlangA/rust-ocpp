use serde::{Deserialize, Serialize};

/// Defines the energy transfer modes that are allowed by the Charging Station.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum EnergyTransferModeEnumType {
    #[serde(rename = "AC_single_phase")]
    ACSinglePhase,
    #[serde(rename = "AC_two_phase")]
    ACTwoPhase,
    #[serde(rename = "AC_three_phase")]
    ACThreePhase,
    DC,
    ACBPT,
    ACBPTDER,
    ACDER,
    DCBPT,
    DCACDP,
    DCACDPBPT,
    WPT,
}
