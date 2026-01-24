use serde::{Deserialize, Serialize};

/// Defines whether certificate needs to be installed or updated.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[derive(Default)]
pub enum CertificateActionEnumType {
    #[default]
    Install,
    Update,
}
