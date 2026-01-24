use serde::{Deserialize, Serialize};

/// The updated reservation status.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub enum ReservationUpdateStatusEnumType {
    /// Reservation update has been accepted.
    #[default]
    Accepted,

    /// Reservation update has failed.
    Failed,

    /// Reservation update has been rejected.
    Rejected,
}
