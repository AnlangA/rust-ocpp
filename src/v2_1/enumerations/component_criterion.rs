use serde::{Deserialize, Serialize};

/// This field contains criteria for components for which a report is requested.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[derive(Default)]
pub enum ComponentCriterionEnumType {
    #[serde(rename = "Active")]
    Active,
    #[serde(rename = "Available")]
    #[default]
    Available,
    #[serde(rename = "Enabled")]
    Enabled,
    #[serde(rename = "Problem")]
    Problem,
}

