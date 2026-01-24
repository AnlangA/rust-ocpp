use serde::{Deserialize, Serialize};

/// This indicates the success or failure of the data transfer.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[derive(Default)]
pub enum DataTransferStatusEnumType {
    #[default]
    Accepted,
    Rejected,
    UnknownMessageId,
    UnknownVendorId,
}
