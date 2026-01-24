use serde::{Deserialize, Serialize};

/// Status indicating whether the Charging Station has unlocked the connector.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub enum UnlockStatusEnumType {
    /// Connector has been unlocked.
    #[default]
    Unlocked,

    /// Failed to unlock the connector.
    UnlockFailed,

    /// Connector is not unlocked because there is still an ongoing authorized transaction.
    OngoingAuthorizedTransaction,

    /// Connector is unknown.
    UnknownConnector,
}
