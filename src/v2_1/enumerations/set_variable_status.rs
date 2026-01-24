use serde::{Deserialize, Serialize};

/// Status of setting a variable.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub enum SetVariableStatusEnumType {
    /// Request has been accepted and the variable has been set.
    #[default]
    Accepted,

    /// Request has been rejected.
    Rejected,

    /// Component specified in the request is unknown.
    UnknownComponent,

    /// Variable specified in the request is unknown.
    UnknownVariable,

    /// Attribute type specified in the request is not supported.
    NotSupportedAttributeType,

    /// Variable has been set but a reboot is required to apply the changes.
    RebootRequired,
}
