use serde::{Deserialize, Serialize};

/// Specifies the event notification type of the message.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum EventNotificationEnumType {
    HardWiredNotification,
    HardWiredMonitor,
    PreconfiguredMonitor,
    CustomMonitor,
}
