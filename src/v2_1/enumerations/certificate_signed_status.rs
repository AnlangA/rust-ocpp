use serde::{Deserialize, Serialize};

/// Returns whether certificate signing has been accepted, otherwise rejected.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[derive(Default)]
pub enum CertificateSignedStatusEnumType {
    #[serde(rename = "Accepted")]
    #[default]
    Accepted,
    #[serde(rename = "Rejected")]
    Rejected,
}

