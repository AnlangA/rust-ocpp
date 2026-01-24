use serde::{Deserialize, Serialize};

/// Format of the message.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum MessageFormatEnumType {
    ASCII,
    HTML,
    URI,
    UTF8,
    QRCODE,
}
