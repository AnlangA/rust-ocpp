use serde::{Deserialize, Serialize};

/// Charging Station indicates if installation was successful.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[derive(Default)]
pub enum InstallCertificateStatusEnumType {
    #[default]
    Accepted,
    Rejected,
    Failed,
}
