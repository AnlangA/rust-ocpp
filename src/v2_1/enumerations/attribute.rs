use serde::{Deserialize, Serialize};

/// Attribute: Actual, Target, MinSet, MaxSet.
/// Defaults to Actual if absent.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[derive(Default)]
pub enum AttributeEnumType {
    #[serde(rename = "Actual")]
    #[default]
    Actual,
    #[serde(rename = "Target")]
    Target,
    #[serde(rename = "MinSet")]
    MinSet,
    #[serde(rename = "MaxSet")]
    MaxSet,
}

