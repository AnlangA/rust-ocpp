use serde::{Deserialize, Serialize};

/// This indicates whether the Charging Station is able to display the message.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[derive(Default)]
pub enum DisplayMessageStatusEnumType {
    #[serde(rename = "Accepted")]
    #[default]
    Accepted,
    #[serde(rename = "NotSupportedMessageFormat")]
    NotSupportedMessageFormat,
    #[serde(rename = "Rejected")]
    Rejected,
    #[serde(rename = "NotSupportedPriority")]
    NotSupportedPriority,
    #[serde(rename = "NotSupportedState")]
    NotSupportedState,
    #[serde(rename = "UnknownTransaction")]
    UnknownTransaction,
    #[serde(rename = "LanguageNotSupported")]
    LanguageNotSupported,
}

