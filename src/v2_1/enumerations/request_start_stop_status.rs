#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "PascalCase")]
pub enum RequestStartStopStatusEnumType {
    #[default]
    Accepted,
    Rejected,
}
