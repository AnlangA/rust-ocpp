use serde::{Deserialize, Serialize};

/// Status of unpublishing a firmware.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub enum UnpublishFirmwareStatusEnumType {
    /// Firmware download is ongoing.
    #[default]
    DownloadOngoing,

    /// No firmware with the given ID was found.
    NoFirmware,

    /// Firmware was successfully unpublished.
    Unpublished,
}
