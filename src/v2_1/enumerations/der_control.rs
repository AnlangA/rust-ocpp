use serde::{Deserialize, Serialize};

/// Type of DER curve
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub enum DERControlEnumType {
    EnterService,
    FreqDroop,
    FreqWatt,
    FixedPFAbsorb,
    FixedPFInject,
    FixedVar,
    Gradients,
    HFMustTrip,
    HFMayTrip,
    HVMustTrip,
    HVMomCess,
    HVMayTrip,
    LimitMaxDischarge,
    LFMustTrip,
    LVMustTrip,
    LVMomCess,
    LVMayTrip,
    PowerMonitoringMustTrip,
    VoltVar,
    VoltWatt,
    WattPF,
    WattVar,
    PowerLimitation,
    PowerTarget,
    PowerFactor,
    VoltageTarget,
    CurrentTarget,
    LoadPriority,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum DERControlStatusEnumType {
    Accepted,
    Rejected,
    NotSupported,
}
