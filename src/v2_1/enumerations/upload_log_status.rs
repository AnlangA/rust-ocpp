use serde::{Deserialize, Serialize};

/// Status of log file upload.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub enum UploadLogStatusEnumType {
    /// Request contains a bad or incomplete message.
    #[default]
    BadMessage,

    /// Charging Station is idle and can process an upload.
    Idle,

    /// Charging Station does not support this operation.
    NotSupportedOperation,

    /// Charging Station has denied permission for this operation.
    PermissionDenied,

    /// Log file has been uploaded successfully.
    Uploaded,

    /// Upload of log file failed.
    UploadFailure,

    /// Log file is being uploaded.
    Uploading,

    /// Upload was accepted but was canceled before completion.
    AcceptedCanceled,
}
