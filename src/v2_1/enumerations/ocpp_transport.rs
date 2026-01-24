use serde::{Deserialize, Serialize};

/// Defines the transport protocol (e.g. SOAP or JSON).
/// Note: SOAP is not supported in OCPP 2.x, but is supported by earlier versions of OCPP.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum OCPPTransportEnumType {
    SOAP,
    JSON,
}
