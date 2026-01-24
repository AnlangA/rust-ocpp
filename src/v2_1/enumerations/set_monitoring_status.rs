use serde::{Deserialize, Serialize};

/// Status of setting a variable monitoring.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub enum SetMonitoringStatusEnumType {
    /// Request has been accepted and the monitoring has been set.
    #[default]
    Accepted,

    /// Component specified in the request is unknown.
    UnknownComponent,

    /// Variable specified in the request is unknown.
    UnknownVariable,

    /// Monitor type specified in the request is not supported.
    UnsupportedMonitorType,

    /// Request has been rejected.
    Rejected,

    /// Value is outside the allowed range.
    OutOfRange,
}
