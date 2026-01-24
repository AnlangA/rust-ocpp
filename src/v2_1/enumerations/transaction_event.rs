use serde::{Deserialize, Serialize};

/// This contains the type of this event.
/// The first TransactionEvent of a transaction SHALL contain: Started
/// The last TransactionEvent of a transaction SHALL contain: Ended
/// All others SHALL contain: Updated
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub enum TransactionEventEnumType {
    /// Transaction has ended.
    #[default]
    Ended,

    /// Transaction has started.
    Started,

    /// Transaction information has been updated.
    Updated,
}
