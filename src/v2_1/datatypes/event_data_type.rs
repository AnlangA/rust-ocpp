use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

use super::component::ComponentType;
use super::custom_data::CustomDataType;
use super::variable::VariableType;
use crate::v2_1::enumerations::event_notification::EventNotificationEnumType;
use crate::v2_1::enumerations::event_trigger::EventTriggerEnumType;

/// EventDataType is used by: NotifyEventRequest
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct EventDataType {
    /// Custom data
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,

    /// Required. Identifies the event. This field can be referred to as a cause by other events.
    #[validate(range(min = 0))]
    pub event_id: i32,

    /// Required. Timestamp of the moment the report was generated.
    pub timestamp: DateTime<Utc>,

    /// Required. Type of trigger for this event, e.g. exceeding a threshold value.
    pub trigger: EventTriggerEnumType,

    /// Optional. Refers to the Id of an event that is considered to be the cause for this event.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(range(min = 0))]
    pub cause: Option<i32>,

    /// Required. Actual value (attributeType Actual) of the variable.
    /// The Configuration Variable ReportingValueSize can be used to limit GetVariableResult.attributeValue,
    /// VariableAttribute.value and EventData.actualValue. The max size of these values will always remain equal.
    #[validate(length(max = 2500))]
    pub actual_value: String,

    /// Optional. Technical (error) code as reported by component.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(max = 50))]
    pub tech_code: Option<String>,

    /// Optional. Technical detail information as reported by component.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(max = 500))]
    pub tech_info: Option<String>,

    /// Optional. Cleared is set to true to report the clearing of a monitored situation, i.e. a 'return to normal'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cleared: Option<bool>,

    /// Optional. If an event notification is linked to a specific transaction, this field can be used to specify its transactionId.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(max = 36))]
    pub transaction_id: Option<String>,

    /// Optional. Identifies the VariableMonitoring which triggered the event.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(range(min = 0))]
    pub variable_monitoring_id: Option<i32>,

    /// Required. Specifies the event notification type of the message.
    pub event_notification_type: EventNotificationEnumType,

    /// Optional. (2.1) Severity associated with the monitor in variableMonitoringId or with the hardwired notification.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(range(min = 0))]
    pub severity: Option<i32>,

    /// Required. Component for which event is notified.
    #[validate(nested)]
    pub component: ComponentType,

    /// Required. Variable for which event is notified.
    #[validate(nested)]
    pub variable: VariableType,
}

impl EventDataType {
    /// Creates a new EventDataType with the minimum required fields.
    pub fn new(
        event_id: i32,
        timestamp: DateTime<Utc>,
        trigger: EventTriggerEnumType,
        actual_value: String,
        event_notification_type: EventNotificationEnumType,
        component: ComponentType,
        variable: VariableType,
    ) -> Self {
        Self {
            custom_data: None,
            event_id,
            timestamp,
            trigger,
            cause: None,
            actual_value,
            tech_code: None,
            tech_info: None,
            cleared: None,
            transaction_id: None,
            variable_monitoring_id: None,
            event_notification_type,
            severity: None,
            component,
            variable,
        }
    }

    /// Sets the custom_data for the EventDataType.
    pub fn with_custom_data(mut self, custom_data: CustomDataType) -> Self {
        self.custom_data = Some(custom_data);
        self
    }

    /// Sets the cause for the EventDataType.
    pub fn with_cause(mut self, cause: i32) -> Self {
        self.cause = Some(cause);
        self
    }

    /// Sets the tech_code for the EventDataType.
    pub fn with_tech_code(mut self, tech_code: String) -> Self {
        self.tech_code = Some(tech_code);
        self
    }

    /// Sets the tech_info for the EventDataType.
    pub fn with_tech_info(mut self, tech_info: String) -> Self {
        self.tech_info = Some(tech_info);
        self
    }

    /// Sets the cleared status for the EventDataType.
    pub fn with_cleared(mut self, cleared: bool) -> Self {
        self.cleared = Some(cleared);
        self
    }

    /// Sets the transaction_id for the EventDataType.
    pub fn with_transaction_id(mut self, transaction_id: String) -> Self {
        self.transaction_id = Some(transaction_id);
        self
    }

    /// Sets the variable_monitoring_id for the EventDataType.
    pub fn with_variable_monitoring_id(mut self, variable_monitoring_id: i32) -> Self {
        self.variable_monitoring_id = Some(variable_monitoring_id);
        self
    }

    /// Sets the severity for the EventDataType.
    pub fn with_severity(mut self, severity: i32) -> Self {
        self.severity = Some(severity);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_event_data_creation() {
        let component = ComponentType::new("TestComponent".to_string());
        let variable = VariableType::new("TestVariable".to_string());

        let event = EventDataType::new(
            1,
            Utc::now(),
            EventTriggerEnumType::Alerting,
            "TestValue".to_string(),
            EventNotificationEnumType::HardWiredNotification,
            component,
            variable,
        );

        assert_eq!(event.event_id, 1);
        assert_eq!(event.actual_value, "TestValue");
        assert_eq!(event.trigger, EventTriggerEnumType::Alerting);
        assert_eq!(
            event.event_notification_type,
            EventNotificationEnumType::HardWiredNotification
        );
    }

    #[test]
    fn test_event_data_type() {
        let component = ComponentType::new("Controller".to_string());
        let variable = VariableType::new("Overcurrent".to_string());
        let timestamp = Utc::now();

        let event_data = EventDataType::new(
            1,
            timestamp,
            EventTriggerEnumType::Alerting,
            "150.5".to_string(),
            EventNotificationEnumType::HardWiredNotification,
            component.clone(),
            variable.clone(),
        )
        .with_cause(0)
        .with_tech_code("E001".to_string())
        .with_tech_info("Overcurrent detected".to_string())
        .with_cleared(false)
        .with_severity(3);

        assert_eq!(event_data.event_id, 1);
        assert_eq!(event_data.timestamp, timestamp);
        assert_eq!(event_data.trigger, EventTriggerEnumType::Alerting);
        assert_eq!(event_data.cause, Some(0));
        assert_eq!(event_data.actual_value, "150.5");
        assert_eq!(event_data.tech_code, Some("E001".to_string()));
        assert_eq!(
            event_data.tech_info,
            Some("Overcurrent detected".to_string())
        );
        assert_eq!(event_data.cleared, Some(false));
        assert_eq!(event_data.severity, Some(3));
        assert_eq!(
            event_data.event_notification_type,
            EventNotificationEnumType::HardWiredNotification
        );
        assert_eq!(event_data.component, component);
        assert_eq!(event_data.variable, variable);
    }

    #[test]
    fn test_event_data_serialization() {
        let component = ComponentType::new("Controller".to_string());
        let variable = VariableType::new("Overcurrent".to_string());
        let timestamp = Utc::now();

        let event_data = EventDataType::new(
            1,
            timestamp,
            EventTriggerEnumType::Alerting,
            "150.5".to_string(),
            EventNotificationEnumType::HardWiredNotification,
            component,
            variable,
        );

        let serialized = serde_json::to_string(&event_data).unwrap();
        let deserialized: EventDataType = serde_json::from_str(&serialized).unwrap();

        assert_eq!(deserialized, event_data);
    }
}
