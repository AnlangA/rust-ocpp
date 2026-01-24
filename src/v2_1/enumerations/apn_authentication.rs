use serde::{Deserialize, Serialize};

/// Authentication method.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum APNAuthenticationEnumType {
    PAP,
    CHAP,
    NONE,
    AUTO,
}
