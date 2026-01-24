use serde::{Deserialize, Serialize};

/// The kind of cost referred to in the message element amount
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[derive(Default)]
pub enum CostKindEnumType {
    #[default]
    CarbonDioxideEmission,
    RelativePricePercentage,
    RenewableGenerationPercentage,
}

