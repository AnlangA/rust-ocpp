use serde::{Deserialize, Serialize};

/// This contains the type of availability change that the Charging Station should perform.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum OperationalStatusEnumType {
    Inoperative,
    Operative,
}
