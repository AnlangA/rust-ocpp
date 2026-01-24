#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "PascalCase")]
pub enum MessageStateEnumType {
    Charging,
    Faulted,
    #[default]
    Idle,
    Unavailable,
    Suspended,
    Discharging,
}
