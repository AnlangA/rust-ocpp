use serde::{Deserialize, Serialize};

/// This indicates whether the Charging Station is able to accept this request.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[derive(Default)]
pub enum GenericDeviceModelStatusEnumType {
    #[default]
    Accepted,
    Rejected,
    NotSupported,
    EmptyResultSet,
}
