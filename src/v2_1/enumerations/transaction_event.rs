use serde::{Deserialize, Serialize};

/// Type of event for a transaction.
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
