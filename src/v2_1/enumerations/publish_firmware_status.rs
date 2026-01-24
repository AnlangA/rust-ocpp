#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "PascalCase")]
pub enum PublishFirmwareStatusEnumType {
    #[default]
    Published,
    DownloadScheduled,
    InvalidChecksum,
    NotDownloaded,
    DownloadFailed,
    Downloaded,
    Downloading,
}
