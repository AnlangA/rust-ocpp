use serde::{Deserialize, Serialize};

/// Source of status: OCSP, CRL
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[derive(Default)]
pub enum CertificateStatusSourceEnumType {
    #[serde(rename = "CRL")]
    CRL,
    #[serde(rename = "OCSP")]
    #[default]
    OCSP,
}

