use serde::{Deserialize, Serialize};

/// This indicates the success or failure of the canceling of a reservation by CSMS.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum CancelReservationStatusEnumType {
    Accepted,
    Rejected,
}
