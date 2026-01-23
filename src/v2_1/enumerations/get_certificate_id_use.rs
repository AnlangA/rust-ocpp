use serde::{Deserialize, Serialize};

/// Indicates the type of the requested certificate(s).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[derive(Default)]
pub enum GetCertificateIdUseEnumType {
    #[serde(rename = "V2GRootCertificate")]
    V2GRootCertificate,
    #[serde(rename = "MORootCertificate")]
    MORootCertificate,
    #[serde(rename = "CSMSRootCertificate")]
    #[default]
    CSMSRootCertificate,
    #[serde(rename = "V2GCertificateChain")]
    V2GCertificateChain,
    #[serde(rename = "ManufacturerRootCertificate")]
    ManufacturerRootCertificate,
    #[serde(rename = "OEMRootCertificate")]
    OEMRootCertificate,
}

