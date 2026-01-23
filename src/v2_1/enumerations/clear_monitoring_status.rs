use serde::{Deserialize, Serialize};

/// Result of the clear request for this monitor, identified by its Id.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[derive(Default)]
pub enum ClearMonitoringStatusEnumType {
    #[serde(rename = "Accepted")]
    #[default]
    Accepted,
    #[serde(rename = "Rejected")]
    Rejected,
    #[serde(rename = "NotFound")]
    NotFound,
}

