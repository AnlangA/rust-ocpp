use serde::{Deserialize, Serialize};

/// Charging Station indicates if it can process the request.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[derive(Default)]
pub enum DeleteCertificateStatusEnumType {
    #[default]
    Accepted,
    Failed,
    NotFound,
}

