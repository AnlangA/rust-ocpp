use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "PascalCase")]
pub enum IslandingDetectionEnumType {
    #[default]
    NoAntiIslandingSupport,
    #[serde(rename = "RoCoF")]
    Rocof,
    #[serde(rename = "UVP_OVP")]
    UvpOvp,
    #[serde(rename = "UFP_OFP")]
    UfpOfp,
    VoltageVectorShift,
    ZeroCrossingDetection,
    OtherPassive,
    ImpedanceMeasurement,
    ImpedanceAtFrequency,
    SlipModeFrequencyShift,
    SandiaFrequencyShift,
    SandiaVoltageShift,
    FrequencyJump,
    #[serde(rename = "RCLQFactor")]
    RCLQFactor,
    OtherActive,
}
