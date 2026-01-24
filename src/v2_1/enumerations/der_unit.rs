use serde::{Deserialize, Serialize};

/// Unit of the Y-axis of DER curve
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
#[derive(Default)]
pub enum DERUnitEnumType {
    #[serde(rename = "Not_Applicable")]
    #[default]
    NotApplicable,
    PctMaxW,
    PctMaxVar,
    PctWAvail,
    PctVarAvail,
    PctEffectiveV,
}
