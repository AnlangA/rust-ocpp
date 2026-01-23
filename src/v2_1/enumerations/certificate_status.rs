use serde::{Deserialize, Serialize};

/// Status of certificate: good, revoked or unknown.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[derive(Default)]
pub enum CertificateStatusEnumType {
    #[serde(rename = "Good")]
    Good,
    #[serde(rename = "Revoked")]
    Revoked,
    #[serde(rename = "Unknown")]
    #[default]
    Unknown,
    #[serde(rename = "Failed")]
    Failed,
}

