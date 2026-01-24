use serde::{Deserialize, Serialize};

/// Attribute: Actual, Target, `MinSet`, `MaxSet`.
/// Defaults to Actual if absent.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[derive(Default)]
pub enum AttributeEnumType {
    #[default]
    Actual,
    Target,
    MinSet,
    MaxSet,
}

