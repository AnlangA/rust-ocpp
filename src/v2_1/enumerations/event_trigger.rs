use serde::{Deserialize, Serialize};

/// Type of trigger for this event, e.g. exceeding a threshold value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[derive(Default)]
pub enum EventTriggerEnumType {
    #[serde(rename = "Alerting")]
    #[default]
    Alerting,
    #[serde(rename = "Delta")]
    Delta,
    #[serde(rename = "Periodic")]
    Periodic,
}

