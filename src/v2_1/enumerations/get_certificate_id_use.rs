use serde::{Deserialize, Serialize};

/// Indicates the type of the requested certificate(s).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[derive(Default)]
pub enum GetCertificateIdUseEnumType {
    V2GRootCertificate,
    MORootCertificate,
    #[default]
    CSMSRootCertificate,
    V2GCertificateChain,
    ManufacturerRootCertificate,
    OEMRootCertificate,
}
