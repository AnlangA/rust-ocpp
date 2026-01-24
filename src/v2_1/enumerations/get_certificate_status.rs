use serde::{Deserialize, Serialize};

/// This indicates whether the charging station was able to retrieve the OCSP certificate status.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum GetCertificateStatusEnumType {
    Accepted,
    Failed,
}
