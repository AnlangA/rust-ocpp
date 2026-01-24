use serde::{Deserialize, Serialize};

/// Reason the message was triggered.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub enum TriggerReasonEnumType {
    /// Transaction has been authorized.
    #[default]
    Authorized,

    /// Cable has been plugged in.
    CablePluggedIn,

    /// Charging rate has changed.
    ChargingRateChanged,

    /// Charging state has changed.
    ChargingStateChanged,

    /// Transaction has been deauthorized.
    Deauthorized,

    /// Energy limit has been reached.
    EnergyLimitReached,

    /// Communication with the EV has been lost.
    EVCommunicationLost,

    /// EV connection timeout.
    EVConnectTimeout,

    /// Clock-aligned meter value.
    MeterValueClock,

    /// Periodic meter value.
    MeterValuePeriodic,

    /// Time limit has been reached.
    TimeLimitReached,

    /// Triggered by a trigger message.
    Trigger,

    /// Triggered by an unlock command.
    UnlockCommand,

    /// Stop has been authorized.
    StopAuthorized,

    /// EV has departed.
    EVDeparted,

    /// EV has been detected.
    EVDetected,

    /// Remote stop command received.
    RemoteStop,

    /// Remote start command received.
    RemoteStart,

    /// Abnormal condition detected.
    AbnormalCondition,

    /// Signed data has been received.
    SignedDataReceived,

    /// Reset command received.
    ResetCommand,
}
