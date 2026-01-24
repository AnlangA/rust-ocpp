use serde::{Deserialize, Serialize};

/// This field contains criteria for components for which a report is requested.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[derive(Default)]
pub enum ComponentCriterionEnumType {
    Active,
    #[default]
    Available,
    Enabled,
    Problem,
}

