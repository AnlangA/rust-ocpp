use serde::{Deserialize, Serialize};

/// This contains the progress status of the firmware installation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[derive(Default)]
pub enum FirmwareStatusEnumType {
    Downloaded,
    DownloadFailed,
    Downloading,
    DownloadScheduled,
    DownloadPaused,
    #[default]
    Idle,
    InstallationFailed,
    Installing,
    Installed,
    InstallRebooting,
    InstallScheduled,
    InstallVerificationFailed,
    InvalidSignature,
    SignatureVerified,
}
