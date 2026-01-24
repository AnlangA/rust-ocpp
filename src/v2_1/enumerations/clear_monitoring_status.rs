use serde::{Deserialize, Serialize};

/// Result of the clear request for this monitor, identified by its Id.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[derive(Default)]
pub enum ClearMonitoringStatusEnumType {
    #[default]
    Accepted,
    Rejected,
    NotFound,
}

