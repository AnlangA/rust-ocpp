use serde::{Deserialize, Serialize};

/// Returns whether certificate signing has been accepted, otherwise rejected.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[derive(Default)]
pub enum CertificateSignedStatusEnumType {
    #[default]
    Accepted,
    Rejected,
}

