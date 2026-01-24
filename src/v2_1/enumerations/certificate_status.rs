use serde::{Deserialize, Serialize};

/// Status of certificate: good, revoked or unknown.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[derive(Default)]
pub enum CertificateStatusEnumType {
    Good,
    Revoked,
    #[default]
    Unknown,
    Failed,
}

