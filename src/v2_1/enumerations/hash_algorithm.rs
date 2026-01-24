use serde::{Deserialize, Serialize};

/// Used algorithms for the hashes provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum HashAlgorithmEnumType {
    SHA256,
    SHA384,
    SHA512,
}
