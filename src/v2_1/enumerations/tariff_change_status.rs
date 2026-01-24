use serde::{Deserialize, Serialize};

/// Status of changing a tariff.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub enum TariffChangeStatusEnumType {
    /// Request has been accepted and the tariff has been changed.
    #[default]
    Accepted,

    /// Request has been rejected.
    Rejected,

    /// Tariff ID specified in the request is invalid.
    InvalidId,
}
