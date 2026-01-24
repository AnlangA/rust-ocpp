use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum DataEnumType {
    #[default]
    String,
    Decimal,
    Integer,
    #[serde(rename = "dateTime")]
    DateTime,
    Boolean,
    #[serde(rename = "OptionList")]
    OptionList,
    #[serde(rename = "SequenceList")]
    SequenceList,
    #[serde(rename = "MemberList")]
    MemberList,
}
