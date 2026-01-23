use serde::{Deserialize, Serialize};

/// The kind of cost referred to in the message element amount
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[derive(Default)]
pub enum CostKindEnumType {
    #[serde(rename = "CarbonDioxideEmission")]
    #[default]
    CarbonDioxideEmission,
    #[serde(rename = "RelativePricePercentage")]
    RelativePricePercentage,
    #[serde(rename = "RenewableGenerationPercentage")]
    RenewableGenerationPercentage,
}

