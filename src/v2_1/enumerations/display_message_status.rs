use serde::{Deserialize, Serialize};

/// This indicates whether the Charging Station is able to display the message.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[derive(Default)]
pub enum DisplayMessageStatusEnumType {
    #[default]
    Accepted,
    NotSupportedMessageFormat,
    Rejected,
    NotSupportedPriority,
    NotSupportedState,
    UnknownTransaction,
    LanguageNotSupported,
}

