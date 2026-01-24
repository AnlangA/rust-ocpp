use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum RegistrationStatusEnumType {
    Accepted,
    Pending,
    Rejected,
}
