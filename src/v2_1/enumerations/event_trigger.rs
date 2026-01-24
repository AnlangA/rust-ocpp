use serde::{Deserialize, Serialize};

/// Type of trigger for this event, e.g. exceeding a threshold value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[derive(Default)]
pub enum EventTriggerEnumType {
    #[default]
    Alerting,
    Delta,
    Periodic,
}

