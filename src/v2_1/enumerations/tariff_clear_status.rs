use serde::{Deserialize, Serialize};

/// Status of clearing a tariff.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub enum TariffClearStatusEnumType {
    /// Request has been accepted and the tariff has been cleared.
    #[default]
    Accepted,

    /// Request has been rejected.
    Rejected,

    /// Tariff ID specified in the request is invalid.
    InvalidId,
}
