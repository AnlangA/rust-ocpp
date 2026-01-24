#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "PascalCase")]
pub enum MonitorEnumType {
    #[default]
    UpperThreshold,
    LowerThreshold,
    Delta,
    Periodic,
    PeriodicClockAligned,
    TargetDelta,
    TargetDeltaRelative,
}
