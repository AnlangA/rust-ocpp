use serde::{Deserialize, Serialize};

/// Type of VPN protocol.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum VPNEnumType {
    /// Internet Key Exchange version 2 protocol.
    IKEv2,

    /// Internet Protocol Security protocol.
    IPSec,

    /// Layer 2 Tunneling Protocol.
    L2TP,

    /// Point-to-Point Tunneling Protocol.
    PPTP,
}
