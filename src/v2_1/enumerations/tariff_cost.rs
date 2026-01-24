use serde::{Deserialize, Serialize};

/// Type of cost: normal or the minimum or maximum cost.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub enum TariffCostEnumType {
    /// Normal cost based on the tariff.
    NormalCost,

    /// Minimum cost that will be billed.
    MinCost,

    /// Maximum cost that will be billed.
    MaxCost,
}
