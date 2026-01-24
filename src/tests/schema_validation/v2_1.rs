use crate::v2_1::datatypes::{CustomDataType, StatusInfoType};
use crate::v2_1::enumerations::CancelReservationStatusEnumType;
use crate::v2_1::messages::cancel_reservation::{
    CancelReservationRequest, CancelReservationResponse,
};
use jsonschema::Validator;
use serde_json::Value;

const SCHEMA_DIR: &str = "src/tests/schema_validation/schemas/v2.1";

// Helper function to validate schema and instance with detailed error reporting
fn validate_schema_instance(
    schema_name: &str,
    instance: Value,
) -> Result<bool, Box<dyn std::error::Error>> {
    let schema_path = format!("{}/{}", SCHEMA_DIR, schema_name);
    let schema_str = std::fs::read_to_string(schema_path)?;
    let schema = serde_json::from_str(&schema_str)?;
    let compiled = Validator::new(&schema).expect("A valid schema");
    let result = compiled.validate(&instance);

    if result.is_err() {
        for error in compiled.iter_errors(&instance) {
            println!("Validation error: {}", error);
            println!("Instance path: {}", error.instance_path);
        }
    }

    Ok(compiled.is_valid(&instance))
}

#[test]
fn test_valid_boot_notification_request() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({
        "reason": "PowerUp",
        "chargingStation": {
            "model": "ModelX",
            "vendorName": "VendorY"
        }
    });

    assert!(validate_schema_instance(
        "BootNotificationRequest.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_invalid_boot_notification_missing_required_field() -> Result<(), Box<dyn std::error::Error>>
{
    let instance = serde_json::json!({
        "reason": "PowerUp",
        // Missing required chargingStation field
    });

    assert!(!validate_schema_instance(
        "BootNotificationRequest.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_valid_authorize_request() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({
        "idToken": {
            "idToken": "ABCD1234",
            "type": "ISO14443"
        }
    });

    assert!(validate_schema_instance("AuthorizeRequest.json", instance)?);
    Ok(())
}

#[test]
fn test_invalid_authorize_request() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({
        "idToken": {
            "idToken": "ABCD1234",
            // Missing required 'type' field
        }
    });

    assert!(!validate_schema_instance(
        "AuthorizeRequest.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_boot_notification_request_additional_field() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({
        "reason": "PowerUp",
        "chargingStation": {
            "model": "ModelX",
            "vendorName": "VendorY"
        },
        "additionalField": "this should NOT be allowed"  // OCPP 2.1 is strict about additional properties
    });

    // The validation should fail because OCPP 2.1 doesn't allow additional properties
    assert!(!validate_schema_instance(
        "BootNotificationRequest.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_valid_boot_notification_request_v2_1() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({
        "reason": "PowerUp",
        "chargingStation": {
            "model": "ModelX",
            "vendorName": "VendorY"
        }
    });

    assert!(validate_schema_instance(
        "BootNotificationRequest.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_valid_boot_notification_response_v2_1() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({
        "currentTime": "2023-10-10T10:10:10Z",
        "interval": 300,
        "status": "Accepted"
    });

    assert!(validate_schema_instance(
        "BootNotificationResponse.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_valid_id_token_type_comprehensive() -> Result<(), Box<dyn std::error::Error>> {
    // Test with all optional fields
    let instance = serde_json::json!({
        "idToken": {
            "additionalInfo": [{
                "additionalIdToken": "TEST123",
                "type": "someType"
            }],
            "idToken": "ABCD1234567890",
            "type": "ISO14443",
            "customData": {
                "vendorId": "TestVendor"
            }
        }
    });
    assert!(validate_schema_instance("AuthorizeRequest.json", instance)?);

    // Test with only required fields
    let instance = serde_json::json!({
        "idToken": {
            "idToken": "ABCD1234567890",
            "type": "Central"
        }
    });
    assert!(validate_schema_instance("AuthorizeRequest.json", instance)?);

    // Test with maximum length strings
    let instance = serde_json::json!({
        "idToken": {
            "idToken": "A".repeat(255),
            "type": "A".repeat(20)
        }
    });
    assert!(validate_schema_instance("AuthorizeRequest.json", instance)?);

    // Test all predefined values
    for type_value in [
        "Central",
        "DirectPayment",
        "eMAID",
        "EVCCID",
        "ISO14443",
        "ISO15693",
        "KeyCode",
        "Local",
        "MacAddress",
        "NoAuthorization",
        "VIN",
    ] {
        let instance = serde_json::json!({
            "idToken": {
                "idToken": "ABCD1234567890",
                "type": type_value
            }
        });
        assert!(validate_schema_instance("AuthorizeRequest.json", instance)?);
    }

    Ok(())
}

#[test]
fn test_invalid_id_token_type() -> Result<(), Box<dyn std::error::Error>> {
    // Test with missing required field
    let instance = serde_json::json!({
        "idToken": {
            "idToken": "ABCD1234567890"
            // Missing required 'type' field
        }
    });
    assert!(!validate_schema_instance(
        "AuthorizeRequest.json",
        instance
    )?);

    // Test with empty additionalInfo array (violates minItems: 1)
    let instance = serde_json::json!({
        "idToken": {
            "additionalInfo": [],
            "idToken": "ABCD1234567890",
            "type": "ISO14443"
        }
    });
    assert!(!validate_schema_instance(
        "AuthorizeRequest.json",
        instance
    )?);

    // Test with too long strings
    let instance = serde_json::json!({
        "idToken": {
            "idToken": "A".repeat(256),
            "type": "ISO14443"
        }
    });
    assert!(!validate_schema_instance(
        "AuthorizeRequest.json",
        instance
    )?);

    let instance = serde_json::json!({
        "idToken": {
            "idToken": "ABCD1234567890",
            "type": "A".repeat(21)  // Type string too long
        }
    });
    assert!(!validate_schema_instance(
        "AuthorizeRequest.json",
        instance
    )?);

    Ok(())
}

#[test]
fn test_valid_adjust_periodic_event_stream_request() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({
        "id": 42,
        "params": {
            "interval": 300,
            "values": 5
        }
    });

    assert!(validate_schema_instance(
        "AdjustPeriodicEventStreamRequest.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_valid_adjust_periodic_event_stream_response() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({
        "status": "Accepted"
    });

    assert!(validate_schema_instance(
        "AdjustPeriodicEventStreamResponse.json",
        instance
    )?);

    // Test with optional fields
    let instance = serde_json::json!({
        "status": "Rejected",
        "statusInfo": {
            "reasonCode": "InvalidParameters",
            "additionalInfo": "Values must be greater than 0"
        }
    });

    assert!(validate_schema_instance(
        "AdjustPeriodicEventStreamResponse.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_invalid_adjust_periodic_event_stream_request() -> Result<(), Box<dyn std::error::Error>> {
    // Test with missing required field
    let instance = serde_json::json!({
        "id": 42
        // Missing required params field
    });

    assert!(!validate_schema_instance(
        "AdjustPeriodicEventStreamRequest.json",
        instance
    )?);

    // Test with negative values
    let instance = serde_json::json!({
        "id": -1,  // Must be >= 0
        "params": {
            "interval": 300,
            "values": 5
        }
    });

    assert!(!validate_schema_instance(
        "AdjustPeriodicEventStreamRequest.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_valid_afrr_signal_request() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({
        "signal": 100,
        "timestamp": "2024-01-01T12:00:00Z"
    });

    assert!(validate_schema_instance(
        "AFRRSignalRequest.json",
        instance
    )?);

    // Test with optional fields
    let instance = serde_json::json!({
        "signal": 100,
        "timestamp": "2024-01-01T12:00:00Z",
        "customData": {
            "vendorId": "TestVendor"
        }
    });

    assert!(validate_schema_instance(
        "AFRRSignalRequest.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_valid_afrr_signal_response() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({
        "status": "Accepted"
    });

    assert!(validate_schema_instance(
        "AFRRSignalResponse.json",
        instance
    )?);

    // Test with optional fields
    let instance = serde_json::json!({
        "status": "Rejected",
        "statusInfo": {
            "reasonCode": "InvalidSignal",
            "additionalInfo": "Signal value out of range"
        }
    });

    assert!(validate_schema_instance(
        "AFRRSignalResponse.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_invalid_afrr_signal_request() -> Result<(), Box<dyn std::error::Error>> {
    // Test with missing required field
    let instance = serde_json::json!({
        "signal": 100
        // Missing required timestamp field
    });

    assert!(!validate_schema_instance(
        "AFRRSignalRequest.json",
        instance
    )?);

    // Test with invalid timestamp format
    let instance = serde_json::json!({
        "signal": 100,
        "timestamp": "invalid-date-time"
    });

    assert!(!validate_schema_instance(
        "AFRRSignalRequest.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_valid_battery_swap_request() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({
        "batteryData": [{
            "evseId": 1,
            "serialNumber": "BATTERY123",
            "soC": 80.5,
            "soH": 95.0
        }],
        "eventType": "BatteryIn",
        "idToken": {
            "idToken": "RFID123",
            "type": "ISO14443"
        },
        "requestId": 42
    });

    assert!(validate_schema_instance(
        "BatterySwapRequest.json",
        instance
    )?);

    // Test with all optional fields
    let instance = serde_json::json!({
        "batteryData": [{
            "evseId": 1,
            "serialNumber": "BATTERY123",
            "soC": 80.5,
            "soH": 95.0,
            "productionDate": "2024-01-01T12:00:00Z",
            "vendorInfo": "Manufacturer XYZ",
            "customData": {
                "vendorId": "TestVendor"
            }
        }],
        "eventType": "BatteryIn",
        "idToken": {
            "idToken": "RFID123",
            "type": "ISO14443"
        },
        "requestId": 42,
        "customData": {
            "vendorId": "TestVendor"
        }
    });

    assert!(validate_schema_instance(
        "BatterySwapRequest.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_valid_battery_swap_response() -> Result<(), Box<dyn std::error::Error>> {
    // Test empty response
    let instance = serde_json::json!({});
    assert!(validate_schema_instance(
        "BatterySwapResponse.json",
        instance
    )?);

    // Test with optional custom data
    let instance = serde_json::json!({
        "customData": {
            "vendorId": "TestVendor"
        }
    });
    assert!(validate_schema_instance(
        "BatterySwapResponse.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_invalid_battery_swap_request() -> Result<(), Box<dyn std::error::Error>> {
    // Test with missing required field
    let instance = serde_json::json!({
        "eventType": "BatteryIn",
        "idToken": {
            "idToken": "RFID123",
            "type": "ISO14443"
        },
        "requestId": 42
        // Missing required batteryData field
    });

    assert!(!validate_schema_instance(
        "BatterySwapRequest.json",
        instance
    )?);

    // Test with empty batteryData array
    let instance = serde_json::json!({
        "batteryData": [],
        "eventType": "BatteryIn",
        "idToken": {
            "idToken": "RFID123",
            "type": "ISO14443"
        },
        "requestId": 42
    });

    assert!(!validate_schema_instance(
        "BatterySwapRequest.json",
        instance
    )?);

    // Test with invalid SoC value
    let instance = serde_json::json!({
        "batteryData": [{
            "evseId": 1,
            "serialNumber": "BATTERY123",
            "soC": 101.0,  // Must be <= 100
            "soH": 95.0
        }],
        "eventType": "BatteryIn",
        "idToken": {
            "idToken": "RFID123",
            "type": "ISO14443"
        },
        "requestId": 42
    });

    assert!(!validate_schema_instance(
        "BatterySwapRequest.json",
        instance
    )?);
    Ok(())
}

#[test]
fn validate_cancel_reservation_request() -> Result<(), Box<dyn std::error::Error>> {
    let test = CancelReservationRequest {
        reservation_id: 42,
        custom_data: None, // Schema doesn't allow custom_data
    };

    let instance = serde_json::to_value(test)?;
    assert!(validate_schema_instance(
        "CancelReservationRequest.json",
        instance
    )?);
    Ok(())
}

#[test]
fn validate_cancel_reservation_response() -> Result<(), Box<dyn std::error::Error>> {
    let test = CancelReservationResponse {
        custom_data: Some(CustomDataType::new("test_vendor".to_string())),
        status: CancelReservationStatusEnumType::Accepted,
        status_info: Some(StatusInfoType {
            reason_code: "NoReservation".to_string(),
            additional_info: Some("No active reservation found".to_string()),
            custom_data: Some(CustomDataType::new("test_vendor".to_string())),
        }),
    };

    let instance = serde_json::to_value(test)?;
    assert!(validate_schema_instance(
        "CancelReservationResponse.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_valid_heartbeat_request() -> Result<(), Box<dyn std::error::Error>> {
    // Test empty request (no required fields)
    let instance = serde_json::json!({});
    assert!(validate_schema_instance(
        "HeartbeatRequest.json",
        instance
    )?);

    // Test with optional custom data
    let instance = serde_json::json!({
        "customData": {
            "vendorId": "TestVendor"
        }
    });
    assert!(validate_schema_instance(
        "HeartbeatRequest.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_valid_heartbeat_response() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({
        "currentTime": "2024-01-15T10:30:00Z"
    });
    assert!(validate_schema_instance(
        "HeartbeatResponse.json",
        instance
    )?);

    // Test with optional custom data
    let instance = serde_json::json!({
        "currentTime": "2024-01-15T10:30:00Z",
        "customData": {
            "vendorId": "TestVendor"
        }
    });
    assert!(validate_schema_instance(
        "HeartbeatResponse.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_invalid_heartbeat_response() -> Result<(), Box<dyn std::error::Error>> {
    // Test with missing required currentTime field
    let instance = serde_json::json!({});
    assert!(!validate_schema_instance(
        "HeartbeatResponse.json",
        instance
    )?);

    // Test with invalid timestamp format
    let instance = serde_json::json!({
        "currentTime": "invalid-date-time"
    });
    assert!(!validate_schema_instance(
        "HeartbeatResponse.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_valid_status_notification_request() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({
        "timestamp": "2024-01-15T10:30:00Z",
        "connectorStatus": "Available",
        "evseId": 1,
        "connectorId": 1
    });
    assert!(validate_schema_instance(
        "StatusNotificationRequest.json",
        instance
    )?);

    // Test with all optional fields
    let instance = serde_json::json!({
        "timestamp": "2024-01-15T10:30:00Z",
        "connectorStatus": "Occupied",
        "evseId": 2,
        "connectorId": 3,
        "customData": {
            "vendorId": "TestVendor"
        }
    });
    assert!(validate_schema_instance(
        "StatusNotificationRequest.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_valid_status_notification_response() -> Result<(), Box<dyn std::error::Error>> {
    // Test empty response
    let instance = serde_json::json!({});
    assert!(validate_schema_instance(
        "StatusNotificationResponse.json",
        instance
    )?);

    // Test with optional custom data
    let instance = serde_json::json!({
        "customData": {
            "vendorId": "TestVendor"
        }
    });
    assert!(validate_schema_instance(
        "StatusNotificationResponse.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_invalid_status_notification_request() -> Result<(), Box<dyn std::error::Error>> {
    // Test with missing required field
    let instance = serde_json::json!({
        "timestamp": "2024-01-15T10:30:00Z",
        "connectorStatus": "Available"
        // Missing required evseId and connectorId
    });
    assert!(!validate_schema_instance(
        "StatusNotificationRequest.json",
        instance
    )?);

    // Test with negative evseId (must be >= 0)
    let instance = serde_json::json!({
        "timestamp": "2024-01-15T10:30:00Z",
        "connectorStatus": "Available",
        "evseId": -1,
        "connectorId": 1
    });
    assert!(!validate_schema_instance(
        "StatusNotificationRequest.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_valid_meter_values_request() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({
        "evseId": 1,
        "meterValue": [{
            "timestamp": "2024-01-15T10:30:00Z",
            "sampledValue": [{
                "value": 100.5,
                "context": "Transaction.Begin"
            }]
        }]
    });
    assert!(validate_schema_instance(
        "MeterValuesRequest.json",
        instance
    )?);

    // Test with optional fields
    let instance = serde_json::json!({
        "evseId": 2,
        "meterValue": [{
            "timestamp": "2024-01-15T10:30:00Z",
            "sampledValue": [{
                "value": 200.5,
                "context": "Transaction.Begin",
                "measurand": "Energy.Active.Import.Register",
                "phase": "L1"
            }],
            "customData": {
                "vendorId": "TestVendor"
            }
        }],
        "customData": {
            "vendorId": "TestVendor"
        }
    });
    assert!(validate_schema_instance(
        "MeterValuesRequest.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_valid_meter_values_response() -> Result<(), Box<dyn std::error::Error>> {
    // Test empty response
    let instance = serde_json::json!({});
    assert!(validate_schema_instance(
        "MeterValuesResponse.json",
        instance
    )?);

    // Test with optional custom data
    let instance = serde_json::json!({
        "customData": {
            "vendorId": "TestVendor"
        }
    });
    assert!(validate_schema_instance(
        "MeterValuesResponse.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_valid_transaction_event_request() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({
        "eventType": "Started",
        "timestamp": "2024-01-15T10:30:00Z",
        "triggerReason": "CablePluggedIn",
        "seqNo": 1,
        "transactionInfo": {
            "transactionId": "TX12345",
            "chargingState": "Charging"
        }
    });
    assert!(validate_schema_instance(
        "TransactionEventRequest.json",
        instance
    )?);

    // Test with more complete data
    let instance = serde_json::json!({
        "eventType": "Updated",
        "timestamp": "2024-01-15T10:30:00Z",
        "triggerReason": "ChargingStateChanged",
        "seqNo": 2,
        "transactionInfo": {
            "transactionId": "TX12345",
            "chargingState": "Charging"
        },
        "meterValue": [{
            "timestamp": "2024-01-15T10:30:00Z",
            "sampledValue": [{
                "value": 100.5,
                "context": "Transaction.Begin"
            }]
        }]
    });
    assert!(validate_schema_instance(
        "TransactionEventRequest.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_valid_transaction_event_response() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({
        "totalCost": 10.5
    });
    assert!(validate_schema_instance(
        "TransactionEventResponse.json",
        instance
    )?);

    // Test with all optional fields
    let instance = serde_json::json!({
        "totalCost": 15.75,
        "customData": {
            "vendorId": "TestVendor"
        }
    });
    assert!(validate_schema_instance(
        "TransactionEventResponse.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_invalid_transaction_event_request() -> Result<(), Box<dyn std::error::Error>> {
    // Test with missing required field
    let instance = serde_json::json!({
        "eventType": "Started",
        "timestamp": "2024-01-15T10:30:00Z",
        "triggerReason": "CablePluggedIn"
        // Missing required seqNo and transactionInfo
    });
    assert!(!validate_schema_instance(
        "TransactionEventRequest.json",
        instance
    )?);

    // Test with invalid seqNo (must be >= 0)
    let instance = serde_json::json!({
        "eventType": "Started",
        "timestamp": "2024-01-15T10:30:00Z",
        "triggerReason": "CablePluggedIn",
        "seqNo": -1,
        "transactionInfo": {
            "transactionId": "TX12345"
        }
    });
    assert!(!validate_schema_instance(
        "TransactionEventRequest.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_valid_reset_request() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({
        "type": "OnIdle",
        "evseId": 1
    });
    assert!(validate_schema_instance(
        "ResetRequest.json",
        instance
    )?);

    // Test with optional customData
    let instance = serde_json::json!({
        "type": "Immediate",
        "customData": {
            "vendorId": "TestVendor"
        }
    });
    assert!(validate_schema_instance(
        "ResetRequest.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_valid_reset_response() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({
        "status": "Accepted"
    });
    assert!(validate_schema_instance(
        "ResetResponse.json",
        instance
    )?);

    // Test with optional customData
    let instance = serde_json::json!({
        "status": "Accepted",
        "customData": {
            "vendorId": "TestVendor"
        }
    });
    assert!(validate_schema_instance(
        "ResetResponse.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_invalid_reset_request() -> Result<(), Box<dyn std::error::Error>> {
    // Test with missing required type field
    let instance = serde_json::json!({});
    assert!(!validate_schema_instance(
        "ResetRequest.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_valid_unlock_connector_request() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({
        "evseId": 1,
        "connectorId": 2
    });
    assert!(validate_schema_instance(
        "UnlockConnectorRequest.json",
        instance
    )?);

    // Test with optional customData
    let instance = serde_json::json!({
        "evseId": 0,
        "connectorId": 1,
        "customData": {
            "vendorId": "TestVendor"
        }
    });
    assert!(validate_schema_instance(
        "UnlockConnectorRequest.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_valid_unlock_connector_response() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({
        "status": "Unlocked"
    });
    assert!(validate_schema_instance(
        "UnlockConnectorResponse.json",
        instance
    )?);

    // Test with statusInfo
    let instance = serde_json::json!({
        "status": "UnlockFailed",
        "statusInfo": {
            "reasonCode": "ConnectorLocked"
        }
    });
    assert!(validate_schema_instance(
        "UnlockConnectorResponse.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_invalid_unlock_connector_request() -> Result<(), Box<dyn std::error::Error>> {
    // Test with missing required connectorId
    let instance = serde_json::json!({
        "evseId": 1
    });
    assert!(!validate_schema_instance(
        "UnlockConnectorRequest.json",
        instance
    )?);

    // Test with negative evseId
    let instance = serde_json::json!({
        "evseId": -1,
        "connectorId": 1
    });
    assert!(!validate_schema_instance(
        "UnlockConnectorRequest.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_valid_clear_display_message_request() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({
        "id": 42
    });
    assert!(validate_schema_instance(
        "ClearDisplayMessageRequest.json",
        instance
    )?);

    // Test with optional customData
    let instance = serde_json::json!({
        "id": 100,
        "customData": {
            "vendorId": "TestVendor"
        }
    });
    assert!(validate_schema_instance(
        "ClearDisplayMessageRequest.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_valid_clear_display_message_response() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({
        "status": "Accepted"
    });
    assert!(validate_schema_instance(
        "ClearDisplayMessageResponse.json",
        instance
    )?);

    // Test with statusInfo (needs reasonCode, max 20 chars)
    let instance = serde_json::json!({
        "status": "Rejected",
        "statusInfo": {
            "reasonCode": "NotFound"
        }
    });
    assert!(validate_schema_instance(
        "ClearDisplayMessageResponse.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_invalid_clear_display_message_request() -> Result<(), Box<dyn std::error::Error>> {
    // Test with missing required id
    let instance = serde_json::json!({});
    assert!(!validate_schema_instance(
        "ClearDisplayMessageRequest.json",
        instance
    )?);

    // Test with negative id
    let instance = serde_json::json!({
        "id": -1
    });
    assert!(!validate_schema_instance(
        "ClearDisplayMessageRequest.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_valid_set_display_message_request() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({
        "message": {
            "id": 42,
            "priority": "AlwaysFront",
            "state": "Charging",
            "message": {
                "format": "UTF8",
                "content": "Please plug in your vehicle"
            },
            "transactionId": "TX12345"
        }
    });
    assert!(validate_schema_instance(
        "SetDisplayMessageRequest.json",
        instance
    )?);

    // Test with optional fields
    let instance = serde_json::json!({
        "message": {
            "id": 100,
            "priority": "InFront",
            "state": "Idle",
            "message": {
                "format": "UTF8",
                "content": "Welcome!"
            },
            "startDateTime": "2024-01-15T10:00:00Z",
            "endDateTime": "2024-01-15T12:00:00Z"
        }
    });
    assert!(validate_schema_instance(
        "SetDisplayMessageRequest.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_valid_set_display_message_response() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({
        "status": "Accepted",
        "statusInfo": {
            "reasonCode": "Success"
        }
    });
    assert!(validate_schema_instance(
        "SetDisplayMessageResponse.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_invalid_set_display_message_request() -> Result<(), Box<dyn std::error::Error>> {
    // Test with missing required message
    let instance = serde_json::json!({});
    assert!(!validate_schema_instance(
        "SetDisplayMessageRequest.json",
        instance
    )?);

    // Test with invalid priority value
    let instance = serde_json::json!({
        "message": {
            "id": 42,
            "priority": "InvalidPriority",
            "state": "Charging",
            "message": {
                "format": "UTF8",
                "content": "Test"
            }
        }
    });
    assert!(!validate_schema_instance(
        "SetDisplayMessageRequest.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_valid_data_transfer_request() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({
        "vendorId": "VendorX",
        "messageId": "UpdateConfig",
        "data": "key=value&key2=value2"
    });
    assert!(validate_schema_instance(
        "DataTransferRequest.json",
        instance
    )?);

    // Test with all fields
    let instance = serde_json::json!({
        "vendorId": "VendorY",
        "messageId": "CustomMessage",
        "data": "custom data here",
        "customData": {
            "vendorId": "TestVendor"
        }
    });
    assert!(validate_schema_instance(
        "DataTransferRequest.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_valid_data_transfer_response() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({
        "status": "Accepted",
        "data": "Response data"
    });
    assert!(validate_schema_instance(
        "DataTransferResponse.json",
        instance
    )?);

    // Test with optional statusInfo
    let instance = serde_json::json!({
        "status": "Rejected",
        "statusInfo": {
            "reasonCode": "UnknownVendorId",
            "additionalInfo": "Vendor not recognized"
        }
    });
    assert!(validate_schema_instance(
        "DataTransferResponse.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_invalid_data_transfer_request() -> Result<(), Box<dyn std::error::Error>> {
    // Test with missing required vendorId
    let instance = serde_json::json!({
        "messageId": "TestMessage"
    });
    assert!(!validate_schema_instance(
        "DataTransferRequest.json",
        instance
    )?);

    // Test with vendorId exceeding max length
    let instance = serde_json::json!({
        "vendorId": "A".repeat(256),  // Max is 255
        "messageId": "TestMessage"
    });
    assert!(!validate_schema_instance(
        "DataTransferRequest.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_valid_get_variables_request() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({
        "getVariableData": [{
            "component": {
                "name": "Transformer",
                "instance": "1",
                "evse": {
                    "id": 1,
                    "connectorId": 2
                }
            },
            "variable": {
                "name": "AlignmentMaxCurrent",
                "instance": "Transformer1"
            }
        }]
    });
    assert!(validate_schema_instance(
        "GetVariablesRequest.json",
        instance
    )?);

    // Test with multiple variables
    let instance = serde_json::json!({
        "getVariableData": [
            {
                "component": {
                    "name": "Controller",
                    "instance": "Main"
                },
                "variable": {
                    "name": "AlignmentMaxCurrent"
                }
            },
            {
                "component": {
                    "name": "Clock"
                },
                "variable": {
                    "name": "ClockAlignment"
                }
            }
        ]
    });
    assert!(validate_schema_instance(
        "GetVariablesRequest.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_valid_get_variables_response() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({
        "getVariableResult": [{
            "attributeStatus": "Accepted",
            "attributeType": "Actual",
            "attributeValue": "30.0",
            "component": {
                "name": "Transformer",
                "instance": "1"
            },
            "variable": {
                "name": "AlignmentMaxCurrent",
                "instance": "Transformer1"
            }
        }]
    });
    assert!(validate_schema_instance(
        "GetVariablesResponse.json",
        instance
    )?);

    // Test with optional statusInfo
    let instance = serde_json::json!({
        "getVariableResult": [{
            "attributeStatus": "Rejected",
            "component": {
                "name": "Clock"
            },
            "variable": {
                "name": "ClockAlignment"
            },
            "attributeStatusInfo": {
                "reasonCode": "UnknownVariable"
            }
        }]
    });
    assert!(validate_schema_instance(
        "GetVariablesResponse.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_invalid_get_variables_request() -> Result<(), Box<dyn std::error::Error>> {
    // Test with missing required getVariableData array
    let instance = serde_json::json!({});
    assert!(!validate_schema_instance(
        "GetVariablesRequest.json",
        instance
    )?);

    // Test with empty getVariableData array
    let instance = serde_json::json!({
        "getVariableData": []
    });
    assert!(!validate_schema_instance(
        "GetVariablesRequest.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_valid_set_variables_request() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({
        "setVariableData": [{
            "attributeValue": "35.5",
            "attributeType": "Actual",
            "component": {
                "name": "Transformer",
                "instance": "1",
                "evse": {
                    "id": 1
                }
            },
            "variable": {
                "name": "AlignmentMaxCurrent"
            }
        }]
    });
    assert!(validate_schema_instance(
        "SetVariablesRequest.json",
        instance
    )?);

    // Test with multiple variables
    let instance = serde_json::json!({
        "setVariableData": [
            {
                "attributeValue": "40.0",
                "attributeType": "Actual",
                "component": {
                    "name": "Controller",
                    "instance": "Main"
                },
                "variable": {
                    "name": "AlignmentMaxCurrent"
                }
            },
            {
                "attributeValue": "300",
                "attributeType": "Actual",
                "component": {
                    "name": "Controller"
                },
                "variable": {
                    "name": "HeartbeatInterval"
                }
            }
        ]
    });
    assert!(validate_schema_instance(
        "SetVariablesRequest.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_valid_set_variables_response() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({
        "setVariableResult": [{
            "attributeStatus": "Accepted",
            "component": {
                "name": "Transformer",
                "instance": "1"
            },
            "variable": {
                "name": "AlignmentMaxCurrent",
                "instance": "Transformer1"
            }
        }]
    });
    assert!(validate_schema_instance(
        "SetVariablesResponse.json",
        instance
    )?);

    // Test with all optional fields
    let instance = serde_json::json!({
        "setVariableResult": [{
            "attributeStatus": "Rejected",
            "attributeType": "Actual",
            "component": {
                "name": "Clock"
            },
            "variable": {
                "name": "ClockAlignment"
            },
            "attributeStatusInfo": {
                "reasonCode": "ReadOnly"
            }
        }]
    });
    assert!(validate_schema_instance(
        "SetVariablesResponse.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_invalid_set_variables_request() -> Result<(), Box<dyn std::error::Error>> {
    // Test with missing required setVariableData
    let instance = serde_json::json!({});
    assert!(!validate_schema_instance(
        "SetVariablesRequest.json",
        instance
    )?);

    // Test with empty setVariableData array
    let instance = serde_json::json!({
        "setVariableData": []
    });
    assert!(!validate_schema_instance(
        "SetVariablesRequest.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_valid_request_start_transaction_request() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({
        "remoteStartId": 12345,
        "idToken": {
            "idToken": "ABC12345",
            "type": "ISO14443"
        }
    });
    assert!(validate_schema_instance(
        "RequestStartTransactionRequest.json",
        instance
    )?);

    // Test with optional fields
    let instance = serde_json::json!({
        "remoteStartId": 67890,
        "idToken": {
            "idToken": "XYZ67890",
            "type": "ISO15693"
        },
        "evseId": 1
    });
    assert!(validate_schema_instance(
        "RequestStartTransactionRequest.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_valid_request_start_transaction_response() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({
        "status": "Accepted"
    });
    assert!(validate_schema_instance(
        "RequestStartTransactionResponse.json",
        instance
    )?);

    // Test with optional transactionId
    let instance = serde_json::json!({
        "status": "Accepted",
        "transactionId": "TX12345"
    });
    assert!(validate_schema_instance(
        "RequestStartTransactionResponse.json",
        instance
    )?);

    // Test with statusInfo
    let instance = serde_json::json!({
        "status": "Rejected",
        "statusInfo": {
            "reasonCode": "Unavailable"
        }
    });
    assert!(validate_schema_instance(
        "RequestStartTransactionResponse.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_invalid_request_start_transaction_request() -> Result<(), Box<dyn std::error::Error>> {
    // Test with missing required remoteStartId
    let instance = serde_json::json!({
        "idToken": {
            "idToken": "ABC12345",
            "type": "ISO14443"
        }
    });
    assert!(!validate_schema_instance(
        "RequestStartTransactionRequest.json",
        instance
    )?);

    // Test with invalid evseId (must be >= 1)
    let instance = serde_json::json!({
        "remoteStartId": 12345,
        "idToken": {
            "idToken": "ABC12345",
            "type": "ISO14443"
        },
        "evseId": 0
    });
    assert!(!validate_schema_instance(
        "RequestStartTransactionRequest.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_valid_request_stop_transaction_request() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({
        "transactionId": "TX12345"
    });
    assert!(validate_schema_instance(
        "RequestStopTransactionRequest.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_valid_request_stop_transaction_response() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({
        "status": "Accepted"
    });
    assert!(validate_schema_instance(
        "RequestStopTransactionResponse.json",
        instance
    )?);

    // Test with optional statusInfo
    let instance = serde_json::json!({
        "status": "Rejected",
        "statusInfo": {
            "reasonCode": "UnknownTransaction"
        }
    });
    assert!(validate_schema_instance(
        "RequestStopTransactionResponse.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_invalid_request_stop_transaction_request() -> Result<(), Box<dyn std::error::Error>> {
    // Test with missing required transactionId
    let instance = serde_json::json!({});
    assert!(!validate_schema_instance(
        "RequestStopTransactionRequest.json",
        instance
    )?);

    // Test with transactionId exceeding max length
    let instance = serde_json::json!({
        "transactionId": "A".repeat(37)
    });
    assert!(!validate_schema_instance(
        "RequestStopTransactionRequest.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_valid_trigger_message_request() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({
        "requestedMessage": "Heartbeat"
    });
    assert!(validate_schema_instance(
        "TriggerMessageRequest.json",
        instance
    )?);

    // Test with optional evse
    let instance = serde_json::json!({
        "requestedMessage": "MeterValues",
        "evse": {
            "id": 1,
            "connectorId": 2
        }
    });
    assert!(validate_schema_instance(
        "TriggerMessageRequest.json",
        instance
    )?);

    // Test with customTrigger
    let instance = serde_json::json!({
        "requestedMessage": "CustomTrigger",
        "customTrigger": "CustomTriggerValue"
    });
    assert!(validate_schema_instance(
        "TriggerMessageRequest.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_valid_trigger_message_response() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({
        "status": "Accepted"
    });
    assert!(validate_schema_instance(
        "TriggerMessageResponse.json",
        instance
    )?);

    // Test with optional statusInfo
    let instance = serde_json::json!({
        "status": "Rejected",
        "statusInfo": {
            "reasonCode": "NotSupported"
        }
    });
    assert!(validate_schema_instance(
        "TriggerMessageResponse.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_invalid_trigger_message_request() -> Result<(), Box<dyn std::error::Error>> {
    // Test with missing required requestedMessage
    let instance = serde_json::json!({});
    assert!(!validate_schema_instance(
        "TriggerMessageRequest.json",
        instance
    )?);

    // Test with invalid evseId
    let instance = serde_json::json!({
        "requestedMessage": "StatusNotification",
        "evse": {
            "id": -1
        }
    });
    assert!(!validate_schema_instance(
        "TriggerMessageRequest.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_valid_clear_cache_request() -> Result<(), Box<dyn std::error::Error>> {
    // Test empty request (no required fields)
    let instance = serde_json::json!({});
    assert!(validate_schema_instance(
        "ClearCacheRequest.json",
        instance
    )?);

    // Test with optional customData
    let instance = serde_json::json!({
        "customData": {
            "vendorId": "TestVendor"
        }
    });
    assert!(validate_schema_instance(
        "ClearCacheRequest.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_valid_clear_cache_response() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({
        "status": "Accepted"
    });
    assert!(validate_schema_instance(
        "ClearCacheResponse.json",
        instance
    )?);

    // Test with optional statusInfo
    let instance = serde_json::json!({
        "status": "Rejected",
        "statusInfo": {
            "reasonCode": "Failed"
        }
    });
    assert!(validate_schema_instance(
        "ClearCacheResponse.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_invalid_clear_cache_response() -> Result<(), Box<dyn std::error::Error>> {
    // Test with missing required status
    let instance = serde_json::json!({});
    assert!(!validate_schema_instance(
        "ClearCacheResponse.json",
        instance
    )?);

    // Test with invalid status value
    let instance = serde_json::json!({
        "status": "InvalidStatus"
    });
    assert!(!validate_schema_instance(
        "ClearCacheResponse.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_valid_change_availability_request() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({
        "operationalStatus": "Inoperative"
    });
    assert!(validate_schema_instance(
        "ChangeAvailabilityRequest.json",
        instance
    )?);

    // Test with optional evse
    let instance = serde_json::json!({
        "operationalStatus": "Operative",
        "evse": {
            "id": 1,
            "connectorId": 2
        }
    });
    assert!(validate_schema_instance(
        "ChangeAvailabilityRequest.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_valid_change_availability_response() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({
        "status": "Accepted"
    });
    assert!(validate_schema_instance(
        "ChangeAvailabilityResponse.json",
        instance
    )?);

    // Test with optional statusInfo
    let instance = serde_json::json!({
        "status": "Scheduled",
        "statusInfo": {
            "reasonCode": "Scheduled"
        }
    });
    assert!(validate_schema_instance(
        "ChangeAvailabilityResponse.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_invalid_change_availability_request() -> Result<(), Box<dyn std::error::Error>> {
    // Test with missing required operationalStatus
    let instance = serde_json::json!({});
    assert!(!validate_schema_instance(
        "ChangeAvailabilityRequest.json",
        instance
    )?);

    // Test with invalid operationalStatus value
    let instance = serde_json::json!({
        "operationalStatus": "InvalidStatus"
    });
    assert!(!validate_schema_instance(
        "ChangeAvailabilityRequest.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_valid_get_local_list_version_request() -> Result<(), Box<dyn std::error::Error>> {
    // Test empty request (no required fields)
    let instance = serde_json::json!({});
    assert!(validate_schema_instance(
        "GetLocalListVersionRequest.json",
        instance
    )?);

    // Test with optional customData
    let instance = serde_json::json!({
        "customData": {
            "vendorId": "TestVendor"
        }
    });
    assert!(validate_schema_instance(
        "GetLocalListVersionRequest.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_valid_get_local_list_version_response() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({
        "versionNumber": 42
    });
    assert!(validate_schema_instance(
        "GetLocalListVersionResponse.json",
        instance
    )?);

    // Test with optional customData
    let instance = serde_json::json!({
        "versionNumber": 100,
        "customData": {
            "vendorId": "TestVendor"
        }
    });
    assert!(validate_schema_instance(
        "GetLocalListVersionResponse.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_invalid_get_local_list_version_response() -> Result<(), Box<dyn std::error::Error>> {
    // Test with missing required versionNumber
    let instance = serde_json::json!({});
    assert!(!validate_schema_instance(
        "GetLocalListVersionResponse.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_valid_clear_charging_profile_request() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({
        "chargingProfileId": 42
    });
    assert!(validate_schema_instance(
        "ClearChargingProfileRequest.json",
        instance
    )?);

    // Test with chargingProfileCriteria
    let instance = serde_json::json!({
        "chargingProfileCriteria": {
            "chargingProfilePurpose": "TxProfile",
            "stackLevel": 1,
            "evseId": 1
        }
    });
    assert!(validate_schema_instance(
        "ClearChargingProfileRequest.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_valid_clear_charging_profile_response() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({
        "status": "Accepted"
    });
    assert!(validate_schema_instance(
        "ClearChargingProfileResponse.json",
        instance
    )?);

    // Test with optional statusInfo
    let instance = serde_json::json!({
        "status": "Unknown",
        "statusInfo": {
            "reasonCode": "NotFound"
        }
    });
    assert!(validate_schema_instance(
        "ClearChargingProfileResponse.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_invalid_clear_charging_profile_request() -> Result<(), Box<dyn std::error::Error>> {
    // Test with negative evseId in criteria
    let instance = serde_json::json!({
        "chargingProfileCriteria": {
            "evseId": -1
        }
    });
    assert!(!validate_schema_instance(
        "ClearChargingProfileRequest.json",
        instance
    )?);

    // Test with negative stackLevel in criteria
    let instance = serde_json::json!({
        "chargingProfileCriteria": {
            "stackLevel": -1
        }
    });
    assert!(!validate_schema_instance(
        "ClearChargingProfileRequest.json",
        instance
    )?);
    Ok(())
}

// We recommend installing an extension to run rust tests.
