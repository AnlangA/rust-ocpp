use serde::{Deserialize, Serialize};

/// Indicates whether EV wants to operate in Dynamic or Scheduled mode.
/// When absent, Scheduled mode is assumed for backwards compatibility.
///
/// ISO 15118-20:
/// ServiceSelectionReq(SelectedEnergyTransferService)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[derive(Default)]
pub enum ControlModeEnumType {
    #[default]
    ScheduledControl,
    DynamicControl,
}
