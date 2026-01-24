use serde::{Deserialize, Serialize};

/// Source of status: OCSP, CRL
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
#[derive(Default)]
pub enum CertificateStatusSourceEnumType {
    CRL,
    #[default]
    OCSP,
}
