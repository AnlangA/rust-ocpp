use serde::{Deserialize, Serialize};

/// This field is ignored, since the OCPP version to use is determined during the websocket handshake.
/// The field is only kept for backwards compatibility with the OCPP 2.0.1 JSON schema.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum OCPPVersionEnumType {
    OCPP12,
    OCPP15,
    OCPP16,
    OCPP20,
    OCPP201,
    OCPP21,
}
