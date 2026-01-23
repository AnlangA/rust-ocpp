use serde::{Deserialize, Serialize};

/// Unit of the Y-axis of DER curve
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[derive(Default)]
pub enum DERUnitEnumType {
    #[serde(rename = "Not_Applicable")]
    #[default]
    NotApplicable,
    #[serde(rename = "PctMaxW")]
    PctMaxW,
    #[serde(rename = "PctMaxVar")]
    PctMaxVar,
    #[serde(rename = "PctWAvail")]
    PctWAvail,
    #[serde(rename = "PctVarAvail")]
    PctVarAvail,
    #[serde(rename = "PctEffectiveV")]
    PctEffectiveV,
}

