#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "UPPERCASE")]
pub enum MobilityNeedsModeEnumType {
    #[default]
    EVCC,
    #[serde(rename = "EVCC_SECC")]
    EVCCSECC,
}
