use serde::{Deserialize, Serialize};

/// Charging Station indicates if installation was successful.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[derive(Default)]
pub enum InstallCertificateStatusEnumType {
    #[serde(rename = "Accepted")]
    #[default]
    Accepted,
    #[serde(rename = "Rejected")]
    Rejected,
    #[serde(rename = "Failed")]
    Failed,
}

