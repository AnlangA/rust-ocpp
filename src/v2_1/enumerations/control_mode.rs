use serde::{Deserialize, Serialize};

/// Indicates whether EV wants to operate in Dynamic or Scheduled mode.
/// When absent, Scheduled mode is assumed for backwards compatibility.
///
/// ISO 15118-20:
/// ServiceSelectionReq(SelectedEnergyTransferService)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[derive(Default)]
pub enum ControlModeEnumType {
    #[serde(rename = "ScheduledControl")]
    #[default]
    ScheduledControl,
    #[serde(rename = "DynamicControl")]
    DynamicControl,
}

