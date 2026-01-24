use serde::{Deserialize, Serialize};

/// Status of firmware update.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub enum UpdateFirmwareStatusEnumType {
    /// Request has been accepted and will be processed.
    #[default]
    Accepted,

    /// Request has been rejected.
    Rejected,

    /// Request has been accepted but was canceled.
    AcceptedCanceled,

    /// Certificate is invalid.
    InvalidCertificate,

    /// Certificate has been revoked.
    RevokedCertificate,
}
