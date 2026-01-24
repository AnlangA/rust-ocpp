use serde::{Deserialize, Serialize};

/// Certificate status information.
/// - if all certificates are valid: return 'Accepted'.
/// - if one of the certificates was revoked, return 'CertificateRevoked'.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum AuthorizeCertificateStatusEnumType {
    Accepted,
    SignatureError,
    CertificateExpired,
    CertificateRevoked,
    NoCertificateAvailable,
    CertChainError,
    ContractCancelled,
}
