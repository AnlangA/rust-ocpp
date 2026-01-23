use serde::{Deserialize, Serialize};

/// Charging Station indicates if it can process the request.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[derive(Default)]
pub enum DeleteCertificateStatusEnumType {
    #[serde(rename = "Accepted")]
    #[default]
    Accepted,
    #[serde(rename = "Failed")]
    Failed,
    #[serde(rename = "NotFound")]
    NotFound,
}

