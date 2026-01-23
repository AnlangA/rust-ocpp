use serde::{Deserialize, Serialize};

/// This indicates the success or failure of the data transfer.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[derive(Default)]
pub enum DataTransferStatusEnumType {
    #[serde(rename = "Accepted")]
    #[default]
    Accepted,
    #[serde(rename = "Rejected")]
    Rejected,
    #[serde(rename = "UnknownMessageId")]
    UnknownMessageId,
    #[serde(rename = "UnknownVendorId")]
    UnknownVendorId,
}

