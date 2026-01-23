use serde::{Deserialize, Serialize};

/// Defines whether certificate needs to be installed or updated.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[derive(Default)]
pub enum CertificateActionEnumType {
    #[serde(rename = "Install")]
    #[default]
    Install,
    #[serde(rename = "Update")]
    Update,
}

